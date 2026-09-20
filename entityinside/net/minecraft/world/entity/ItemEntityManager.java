package net.minecraft.world.entity;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandles.Lookup;
import java.util.function.Consumer;

import ca.spottedleaf.moonrise.common.util.TickThread;
import io.papermc.paper.entity.activation.ActivationRange;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.tags.FluidTags;
import net.minecraft.util.Mth;
import net.minecraft.world.TickRateManager;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.Vec3;
import org.bukkit.craftbukkit.event.CraftEventFactory;
import org.bukkit.event.entity.EntityRemoveEvent;

/**
 * ITEM-MANAGER (TASK-395 mega-round, agent J — lever items_manager).
 *
 * Полная замена диспетч-структуры item-фазы: ItemEntity больше не идут через
 * общий entity-tick dispatch (ServerLevel.lambda$tick$4 → Level.guardEntityTick
 * → ServerLevel.tickNonPassenger → Entity.tick → ItemEntity.tick), а тикаются
 * батч-фазами из RegionTickOps по per-slot плотным массивам.
 *
 * КОНТРАКТ ПАРИТИ (javap-верифицировано против purpur-1.21.10, mojmap):
 *  - Гейты lambda$tick$4: isRemoved → TickRateManager.isEntityFrozen →
 *    checkDespawn → vehicle-гейт (alive vehicle с этим пассажиром → return;
 *    иначе stopRiding) — тело воспроизведено 1:1 (offsets 0..64).
 *  - Гейты tickNonPassenger: setOldPosAndRot → tickCount++/totalEntityAge++ →
 *    ActivationRange.checkIfActive → active ? {tick-тело; postTick} :
 *    inactiveTick — воспроизведено; profiler push/incrementCounter и
 *    currentlyTickingEntity (диагностика крашей, package-private в
 *    net.minecraft.server.level) опущены — не имеют игровой семантики.
 *  - ItemEntity.tick — побайтная реплика (offsets 0..588): getItem().isEmpty
 *    → discard(DESPAWN); Entity.tick-гейт (despawnTime, private final —
 *    по MethodHandle findGetter) + baseTick(); pickupDelay--; xo/yo/zo;
 *    water/lava(>0.1) → setFluidMovement-реплика (0.99/0.95, +0.005 если
 *    y<0.06) иначе applyGravity; noPhysics=!noCollision(deflate(1e-7)) +
 *    moveTowardsClosestSpace; move-гейт (onGround && hdSqr<=1e-5 &&
 *    (tickCount+id)%4!=0 → skip); applyEffectsFromBlocks; friction
 *    (TriState.FALSE→1, onGround→block.getFriction()*0.98); multiply(f,0.98,f);
 *    bounce y*-0.5; moved по floor(xo/yo/zo); k=moved?2:40;
 *    tickCount%k==0 && !isClientSide && isMergable → ВАНИЛЬНЫЙ приватный
 *    mergeWithNeighbours по MethodHandle (ноль репликации мердж-логики,
 *    вызов в точном ванильном месте цикла — порядок и позиции бит-в-бит);
 *    age++ (гейт -32768); hasImpulse|=updateInWaterStateAndDoFluidPushing();
 *    delta-sqr(v0)>0.01 → hasImpulse; age>=despawnRate → ванильный
 *    ItemDespawnEvent → cancel? age=0 : discard(DESPAWN).
 *  - guardEntityTick catch-семантика: Throwable → лог + ServerExceptionEvent +
 *    discard(Cause.DISCARD) (CraftBukkit body, offsets 10..110), НЕ rethrow.
 *  - Ванильный порядок внутри секции: items маршрутизируются в плотный массив
 *    в порядке снапшота EntityTickList; item-фаза выполняется на том же
 *    бакет-потоке, что и ванильные сущности секции; item-vs-моб интерлив
 *    внутри бакета принадлежит тому же accepted interleave-классу, что и
 *    кросс-бакет параллельный тик (S7-155/RECON-15).
 *  - ItemEntity с пассажирами (патология) НЕ маршрутизируются — остаются в
 *    ванильном consumer-пути (см. RegionTickOps.fill).
 *  - Применение к ядру — ТОЛЬКО ванильные методы (move/baseTick/discard/
 *    CraftEventFactory/ActivationRange); мутации EntityTickList — ванильными
 *    EntityCallbacks через существующие retarget'ы (items не удаляются
 *    напрямую; discard триггерит ванильный flow).
 *
 * FAIL-CLOSED: если MethodHandle-резолв не удался (READY=false) или класс не
 * определён в kernel loader — RegionTickOps.itemsManagerArmed() возвращает
 * false и items идут ванильным dispatch бит-в-бит.
 *
 * ТЕЛЕМЕТРИЯ: пары "tickSlot items=... slot=..." печатаются каждые
 * TELEMETRY_INTERVAL вызовов tickSlot (ответ на вопрос "жив ли конвейер").
 */
public final class ItemEntityManager {

    private static final boolean ENABLED =
            "items_manager".equals(trimToEmpty(System.getenv("CRUSSTY_LEVER_FLAG")));

    private static final MethodHandle MH_MERGE_WITH_NEIGHBOURS;
    private static final MethodHandle MH_DESPAWN_RATE;  // ItemEntity.despawnRate (private int)
    private static final MethodHandle MH_DESPAWN_TIME;  // Entity.despawnTime (private final int)

    /** true после успешного статического резолва всех MethodHandle. */
    private static final boolean READY;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    private static final int TELEMETRY_INTERVAL = 1200;
    private static long telemetryCounter = 0;

    static {
        boolean ok = false;
        MethodHandle merge = null;
        MethodHandle rate = null;
        MethodHandle time = null;
        try {
            Lookup itemLookup = MethodHandles.privateLookupIn(ItemEntity.class, MethodHandles.lookup());
            merge = itemLookup.unreflect(ItemEntity.class.getDeclaredMethod("mergeWithNeighbours"));
            rate = itemLookup.findGetter(ItemEntity.class, "despawnRate", int.class);
            Lookup entityLookup = MethodHandles.privateLookupIn(Entity.class, MethodHandles.lookup());
            time = entityLookup.findGetter(Entity.class, "despawnTime", int.class);
            ok = true;
        } catch (Throwable t) {
            LOG.severe("[crussty-plugin] items_manager: MethodHandle resolve failed: " + t);
        }
        MH_MERGE_WITH_NEIGHBOURS = merge;
        MH_DESPAWN_RATE = rate;
        MH_DESPAWN_TIME = time;
        READY = ok;
        if (READY) {
            LOG.info("[crussty-plugin] items_manager: bridge ready (enabled=" + ENABLED + ")");
        }
    }

    private ItemEntityManager() {}

    private static String trimToEmpty(String s) {
        return s == null ? "" : s.trim();
    }

    /** Gate для RegionTickOps: армировать ли item-маршрутизацию. */
    public static boolean armed() {
        return ENABLED && READY;
    }

    /**
     * Item-фаза одного бакета: батч-конвейер gather(готов) → движение+age/despawn
     * (merge ванильный inline) → применение к ядру ванильными методами.
     * Вызывается из RegionTickOps.tickBucket(slot) на потоке бакета.
     *
     * @param items    плотный массив ItemEntity (ванильный порядок снапшота)
     * @param n        длина валидной части
     * @param consumer ванильный consumer (fallback-путь, не используется для
     *                 обычных items — только семантическая страховка)
     */
    public static void tickSlot(ItemEntity[] items, int n, Consumer<Entity> consumer) {
        if (n <= 0) {
            return;
        }
        TickThread.ensureTickThread("items_manager off-main");
        Level level = items[0].level();
        TickRateManager trm = level instanceof ServerLevel serverLevel
                ? serverLevel.tickRateManager()
                : null;
        if ((++telemetryCounter % TELEMETRY_INTERVAL) == 0L) {
            LOG.info("[crussty-plugin] items_manager: telemetry calls=" + telemetryCounter
                    + " lastBatch=" + n);
        }
        for (int i = 0; i < n; i++) {
            tickOne(items[i], trm, consumer);
        }
    }

    private static void tickOne(ItemEntity e, TickRateManager trm, Consumer<Entity> consumer) {
        // ---- guardEntityTick (CraftBukkit body): try { dispatch } catch { log+event+discard } ----
        try {
            dispatch(e, trm, consumer);
        } catch (Throwable throwable) {
            try {
                String worldName;
                try {
                    worldName = e.level().getWorld().getName();
                } catch (Throwable t2) {
                    worldName = "unknown";
                }
                LOG.severe("[crussty-plugin] items_manager: Entity threw exception at "
                        + worldName + ":" + e.getX() + "," + e.getY() + "," + e.getZ());
                try {
                    org.bukkit.Bukkit.getPluginManager().callEvent(
                            new com.destroystokyo.paper.event.server.ServerExceptionEvent(
                                    new com.destroystokyo.paper.exception.ServerInternalException(
                                            "Entity threw exception at " + worldName + ":"
                                                    + e.getX() + "," + e.getY() + "," + e.getZ(),
                                            throwable)));
                } catch (Throwable ignored) {
                    // event dispatch is diagnostics-only; vanilla would also survive its absence
                }
                e.discard(EntityRemoveEvent.Cause.DISCARD);
            } catch (Throwable fatal) {
                LOG.severe("[crussty-plugin] items_manager: exception handler rethrew: " + fatal);
            }
        }
    }

    /** Реплика ServerLevel.lambda$tick$4 + tickNonPassenger (без profiler/диагностики). */
    private static void dispatch(ItemEntity e, TickRateManager trm, Consumer<Entity> consumer) {
        // --- lambda$tick$4 offsets 0..64 ---
        if (e.isRemoved()) {
            return;
        }
        if (trm != null && trm.isEntityFrozen(e)) {
            return;
        }
        e.checkDespawn();
        Entity vehicle = e.getVehicle();
        if (vehicle != null) {
            if (!vehicle.isRemoved() && vehicle.hasPassenger(e)) {
                return;
            }
            e.stopRiding();
        }
        // --- tickNonPassenger offsets 22..95 (без profiler/памятки) ---
        TickThread.ensureTickThread("Cannot tick an entity off-main");
        e.setOldPosAndRot();
        e.tickCount++;
        e.totalEntityAge++;
        boolean active = ActivationRange.checkIfActive(e);
        if (active) {
            tickBody(e);
            e.postTick();
        } else {
            e.inactiveTick();
        }
    }

    /**
     * Побайтная реплика ItemEntity.tick (purpur-1.21.10, offsets 0..588) с
     * единственной структурной разницей: getItem() hoisted 1x/тик (SynchedEntityData
     * не может измениться внутри тела — сеттеров в пути нет, мерж выполняет
     * ванильный private-код). merge/despawn — ванильные вызовы.
     */
    private static void tickBody(ItemEntity e) {
        ItemStack stack = e.getItem(); // offset 0 — единственный synched-read на тик
        if (stack.isEmpty()) {
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
            return;
        }
        // Entity.tick (offsets 20..30): despawnTime-гейт + baseTick
        int despawnTime = getDespawnTime(e);
        if (despawnTime >= 0 && e.totalEntityAge >= despawnTime) {
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
        } else {
            e.baseTick();
        }
        // offsets 24..48
        if (e.pickupDelay > 0 && e.pickupDelay != 32767) {
            e.pickupDelay--;
        }
        // offsets 51..72
        e.xo = e.getX();
        e.yo = e.getY();
        e.zo = e.getZ();
        Vec3 vec3 = e.getDeltaMovement();
        // offsets 80..139: fluid-movement / gravity
        if (e.isInWater() && e.getFluidHeight(FluidTags.WATER) > 0.10000000149011612D) {
            // setUnderwaterMovement: setFluidMovement(0.9900000095367432)
            setFluidMovement(e, 0.9900000095367432D);
        } else if (e.isInLava() && e.getFluidHeight(FluidTags.LAVA) > 0.10000000149011612D) {
            // setUnderLavaMovement: setFluidMovement(0.949999988079071)
            setFluidMovement(e, 0.949999988079071D);
        } else {
            e.applyGravity();
        }
        // offsets 140..225: noPhysics / moveTowardsClosestSpace
        if (e.level().isClientSide()) {
            e.noPhysics = false;
        } else {
            e.noPhysics = !e.level().noCollision(e, e.getBoundingBox().deflate(1.0E-7D));
            if (e.noPhysics) {
                e.moveTowardsClosestSpace(e.getX(),
                        (e.getBoundingBox().minY + e.getBoundingBox().maxY) / 2.0D, e.getZ());
            }
        }
        // offsets 226..270: move-гейт
        if (e.onGround() && e.getDeltaMovement().horizontalDistanceSqr() <= 9.999999747378752E-6D
                && (e.tickCount + e.getId()) % 4 != 0) {
            // skip move (vanilla branch)
        } else {
            e.move(MoverType.SELF, e.getDeltaMovement());
        }
        // offset 272
        e.applyEffectsFromBlocks();
        // offsets 276..341: friction
        float f = 0.98F;
        if (e.frictionState == net.kyori.adventure.util.TriState.FALSE) {
            f = 1.0F;
        } else if (e.onGround()) {
            f = e.level().getBlockState(e.getBlockPosBelowThatAffectsMyMovement()).getBlock()
                    .getFriction() * 0.98F;
        }
        e.setDeltaMovement(e.getDeltaMovement().multiply(f, 0.9800000190734863D, f));
        // offsets 342..375: bounce
        if (e.onGround()) {
            Vec3 vec31 = e.getDeltaMovement();
            if (vec31.y < 0.0D) {
                e.setDeltaMovement(vec31.multiply(1.0D, -0.5D, 1.0D));
            }
        }
        // offsets 376..473: merge window (ВАНИЛЬНЫЙ private merge по MethodHandle)
        boolean moved = Mth.floor(e.xo) != Mth.floor(e.getX())
                || Mth.floor(e.yo) != Mth.floor(e.getY())
                || Mth.floor(e.zo) != Mth.floor(e.getZ());
        int k = moved ? 2 : 40;
        if (e.tickCount % k == 0 && !e.level().isClientSide() && isMergable(e, stack)) {
            invokeVanillaMerge(e);
        }
        // offsets 474..498: age
        if (e.age != -32768) {
            e.age++;
        }
        // offsets 499..506
        e.hasImpulse = e.hasImpulse | e.updateInWaterStateAndDoFluidPushing();
        // offsets 507..543
        if (!e.level().isClientSide()) {
            double d0 = e.getDeltaMovement().subtract(vec3).lengthSqr();
            if (d0 > 0.01D) {
                e.hasImpulse = true;
            }
        }
        // offsets 544..588: despawn
        if (!e.level().isClientSide() && e.age >= getDespawnRate(e)) {
            if (CraftEventFactory.callItemDespawnEvent(e).isCancelled()) {
                e.age = 0;
                return;
            }
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
        }
    }

    /** Реплика приватного setFluidMovement(double) (offsets 0..45). */
    private static void setFluidMovement(ItemEntity e, double mult) {
        Vec3 vec3 = e.getDeltaMovement();
        e.setDeltaMovement(vec3.x * mult,
                vec3.y + (vec3.y < 0.05999999865889549D ? 0.004999999888241291D : 0.0D),
                vec3.z * mult);
    }

    /** Реплика приватного isMergable() (offsets 0..59) c hoisted stack. */
    private static boolean isMergable(ItemEntity e, ItemStack stack) {
        return e.isAlive()
                && e.pickupDelay != 32767
                && e.age != -32768
                && e.age < getDespawnRate(e)
                && stack.getCount() < stack.getMaxStackSize();
    }

    private static int getDespawnRate(ItemEntity e) {
        try {
            return (int) MH_DESPAWN_RATE.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    private static int getDespawnTime(Entity e) {
        try {
            return (int) MH_DESPAWN_TIME.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    private static void invokeVanillaMerge(ItemEntity e) {
        try {
            MH_MERGE_WITH_NEIGHBOURS.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }
}
