package net.minecraft.world.entity;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandles.Lookup;
import java.util.concurrent.ConcurrentHashMap;

import ca.spottedleaf.moonrise.common.util.TickThread;
import io.papermc.paper.entity.activation.ActivationRange;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.tags.FluidTags;
import net.minecraft.util.Mth;
import net.minecraft.world.TickRateManager;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.HitResult;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.CollisionContext;
import org.bukkit.craftbukkit.event.CraftEventFactory;
import org.bukkit.event.entity.EntityRemoveEvent;

/**
 * ITEM-SUBSYS2 (TASK-397 mega-round-2, agent J — lever items_subsys2).
 *
 * Эволюция items_manager (round-1): полная замена item-фазы СОБСТВЕННЫМ
 * индексом. Два отличия от round-1:
 *
 *  1) MERGE БЕЗ BROADPHASE: ванильный mergeWithNeighbours ищет кандидатов через
 *     level.getEntitiesOfClass(ItemEntity.class, bb.inflate(itemMerge), ...) =
 *     скан ВСЕЙ 16³-секции (Moonrise ClassInstanceMultiMap) на каждый
 *     merge-gate тик. Здесь кандидатов даёт пространственный индекс 1.0-grid
 *     на rust-стороне (src/items_index.rs, плоские массивы + RegisterNatives):
 *     idxQuery возвращает id-кандидаты в scratch int[] (ноль аллокаций,
 *     ноль fastutil), точные ванильные фильтры (level, AABB.intersects,
 *     other != this && other.isMergable(), walls-fix через clipDirect) и
 *     ванильный tryToMerge (MethodHandle) — множество кандидатов и исход
 *     мерджа = ванильным. Любой отказ индекса (rc<0, структурный код) →
 *     ВАНИЛЬНЫЙ mergeWithNeighbours по MethodHandle для этого вызова.
 *
 *  2) ОДИН ПРОХОД В КРИТ-СЕКЦИИ: RegionTickOps.tickBucket тикает items
 *     инлайн (tickOne) в том же проходе, где ванильный consumer тикает
 *     остальные сущности — порядок внутри слота = ванильный порядок снапшота
 *     EntityTickList (сильнее round-1: items шли отдельной фазой до общего
 *     цикла). Никаких per-slot item-массивов, двойного обхода и iarr-hygiene.
 *
 * Актуальность индекса: onTickingStart/onTickingEnd (единственные call-sites
 * EntityTickList.add/remove) + lazy indexAdd на первом manager-тике (само-
 * исцеление для items, заспавненных до армирования) + idxSetCell ТОЛЬКО на
 * тиках пересечения границы блока (moved = floor-change — именно он; осевшие
 * 99% популяции не делают ни одного native-вызова на тик).
 *
 * ПАРИТЕТ (наследован round-1, javap purpur-1.21.10): гейты lambda$tick$4 /
 * tickNonPassenger / тело ItemEntity.tick [0..588] / guardEntityTick catch —
 * без изменений; despawn/pickup/ItemDespawnEvent — ванильные.
 *
 * FAIL-CLOSED: ENABLED (env) && READY (MethodHandle resolve) && nativeOk
 * (idxProbe magic) && !indexBroken → иначе 100% ванильный путь.
 */
public final class ItemEntityManager {

    /**
     * TASK-399-B (cmp399_shard): gate widened to the cmp399_* lever family.
     * NOTE (source/binary delta): the committed build/…/ItemEntityManager.class
     * is the round-398-J javac artifact whose baked gate string is the bare
     * "items_subsys2"; for cmp399_* levers the rust side CP-patches that one
     * Utf8 constant at define time (src/classfile.rs patch_utf8_gate — bytecode
     * transparent, cp indices unchanged), so the shipped binary arms under
     * cmp399_shard exactly. Recompiling THIS source yields a superset gate
     * (every cmp399_*), semantics-compatible with the runtime patch.
     */
    private static boolean leverEnabled() {
        String f = trimToEmpty(System.getenv("CRUSSTY_LEVER_FLAG"));
        return "items_subsys2".equals(f) || f.startsWith("cmp399_");
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x1D3A;

    private static final MethodHandle MH_TRY_TO_MERGE;          // ItemEntity.tryToMerge(ItemEntity) private
    private static final MethodHandle MH_MERGE_WITH_NEIGHBOURS; // vanilla fallback
    private static final MethodHandle MH_DESPAWN_RATE;          // ItemEntity.despawnRate (private int)
    private static final MethodHandle MH_DESPAWN_TIME;          // Entity.despawnTime (private final int)

    /** true после успешного статического резолва всех MethodHandle. */
    private static final boolean READY;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/items_index.rs, RegisterNatives после define) ----
    private static native int idxProbe();
    private static native int idxInsert(int id, int lid, int cx, int cy, int cz);
    private static native int idxSetCell(int id, int lid, int cx, int cy, int cz);
    private static native int idxRemove(int id);
    private static native int idxQuery(double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int lid, int[] out);

    private static volatile boolean nativeOk;
    private static volatile boolean indexBroken;

    /** id → entity (плотный массив, grow ×2; ids реиспользуются через freeIds). */
    private static ItemEntity[] byId = new ItemEntity[1024];
    private static int idTop = 0;
    private static int[] freeIds = new int[256];
    private static int freeTop = 0;
    /** entity → id-box. Пишется только на main между фазами; читается воркерами. */
    private static final ConcurrentHashMap<ItemEntity, int[]> idMap = new ConcurrentHashMap<>();
    private static final Object ID_LOCK = new Object();

    /** Merge-query scratch: per-thread, grow-only, ноль аллокаций в steady-state. */
    private static final ThreadLocal<int[]> SCRATCH =
            ThreadLocal.withInitial(() -> new int[128]);

    private static final int TELEMETRY_INTERVAL = 24000;
    private static long telemetryCounter = 0;

    static {
        boolean ok = false;
        MethodHandle tryMerge = null;
        MethodHandle merge = null;
        MethodHandle rate = null;
        MethodHandle time = null;
        try {
            Lookup itemLookup = MethodHandles.privateLookupIn(ItemEntity.class, MethodHandles.lookup());
            tryMerge = itemLookup.unreflect(ItemEntity.class.getDeclaredMethod("tryToMerge", ItemEntity.class));
            merge = itemLookup.unreflect(ItemEntity.class.getDeclaredMethod("mergeWithNeighbours"));
            rate = itemLookup.findGetter(ItemEntity.class, "despawnRate", int.class);
            Lookup entityLookup = MethodHandles.privateLookupIn(Entity.class, MethodHandles.lookup());
            time = entityLookup.findGetter(Entity.class, "despawnTime", int.class);
            ok = true;
        } catch (Throwable t) {
            LOG.severe("[crussty-plugin] items_subsys2: MethodHandle resolve failed: " + t);
        }
        MH_TRY_TO_MERGE = tryMerge;
        MH_MERGE_WITH_NEIGHBOURS = merge;
        MH_DESPAWN_RATE = rate;
        MH_DESPAWN_TIME = time;
        READY = ok;
        if (READY) {
            LOG.info("[crussty-plugin] items_subsys2: bridge ready (enabled=" + ENABLED + ")");
        }
    }

    private ItemEntityManager() {}

    private static String trimToEmpty(String s) {
        return s == null ? "" : s.trim();
    }

    /** Ленивая проверка нативов (первый armed(); до регистрации — Throwable → false, ретрай). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (ItemEntityManager.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = idxProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Gate для RegionTickOps: армировать ли item-маршрутизацию. */
    public static boolean armed() {
        return ENABLED && READY && !indexBroken && probeOnce();
    }

    // ------------------------------------------------------------------
    // Index lifecycle (main-thread; фазы region-tick защищены deferral)
    // ------------------------------------------------------------------

    /** Положить item в индекс (idempotent). Вызывается из onTickingStart,
     *  phase-4 drain и лениво с первого manager-тика (в т.ч. с воркера). */
    static void indexAdd(ItemEntity e) {
        if (!READY || indexBroken) {
            return;
        }
        if (idMap.containsKey(e)) {
            return;
        }
        synchronized (ID_LOCK) {
            if (indexBroken || idMap.containsKey(e)) {
                return;
            }
            int id;
            if (freeTop > 0) {
                id = freeIds[--freeTop];
            } else {
                if (idTop == byId.length) {
                    byId = java.util.Arrays.copyOf(byId, byId.length * 2);
                }
                id = idTop++;
            }
            int lid = System.identityHashCode(e.level());
            int rc = idxInsert(id, lid, Mth.floor(e.getX()), Mth.floor(e.getY()), Mth.floor(e.getZ()));
            if (rc != 0) {
                indexBroken = true;
                return; // id не занят (rollback выше) — merge уйдёт в vanilla
            }
            byId[id] = e;
            idMap.put(e, new int[] {id});
        }
    }

    /** Снять item с индекса. Вызывается из onTickingEnd. */
    static void indexRemove(ItemEntity e) {
        int[] box = idMap.remove(e);
        if (box == null) {
            return;
        }
        int id = box[0];
        int rc = idxRemove(id);
        byId[id] = null;
        synchronized (ID_LOCK) {
            if (freeTop == freeIds.length) {
                freeIds = java.util.Arrays.copyOf(freeIds, Math.max(16, freeTop * 2));
            }
            freeIds[freeTop++] = id;
        }
        if (rc != 0) {
            indexBroken = true; // dangling chain — на следующий тик весь путь в vanilla
        }
    }

    // ------------------------------------------------------------------
    // Tick entry (RegionTickOps.tickBucket — инлайн, один проход)
    // ------------------------------------------------------------------

    /**
     * Полный manager-тик одного ItemEntity: guardEntityTick catch-семантика +
     * dispatch-реплика + побайтная реплика ItemEntity.tick с merge из
     * собственного индекса. Вызывается вместо vanilla consumer.
     */
    public static void tickOne(ItemEntity e, TickRateManager trm) {
        if ((++telemetryCounter % TELEMETRY_INTERVAL) == 0L) {
            LOG.info("[crussty-plugin] items_subsys2: telemetry calls=" + telemetryCounter);
        }
        // ---- guardEntityTick (CraftBukkit body): try { dispatch } catch { log+event+discard } ----
        try {
            dispatch(e, trm);
        } catch (Throwable throwable) {
            try {
                String worldName;
                try {
                    worldName = e.level().getWorld().getName();
                } catch (Throwable t2) {
                    worldName = "unknown";
                }
                LOG.severe("[crussty-plugin] items_subsys2: Entity threw exception at "
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
                LOG.severe("[crussty-plugin] items_subsys2: exception handler rethrew: " + fatal);
            }
        }
    }

    /** Реплика ServerLevel.lambda$tick$4 + tickNonPassenger (без profiler/диагностики). */
    private static void dispatch(ItemEntity e, TickRateManager trm) {
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
     * Побайтная реплика ItemEntity.tick (purpur-1.21.10, offsets 0..588):
     * getItem() hoisted 1x/тик; merge — из собственного индекса (fallback —
     * ванильный private mergeWithNeighbours); после merge-окна — обновление
     * клетки индекса ТОЛЬКО при пересечении границы блока (moved).
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
            setFluidMovement(e, 0.9900000095367432D);
        } else if (e.isInLava() && e.getFluidHeight(FluidTags.LAVA) > 0.10000000149011612D) {
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
        // offsets 376..473: merge window (кандидаты из собственного индекса)
        boolean moved = Mth.floor(e.xo) != Mth.floor(e.getX())
                || Mth.floor(e.yo) != Mth.floor(e.getY())
                || Mth.floor(e.zo) != Mth.floor(e.getZ());
        int k = moved ? 2 : 40;
        if (e.tickCount % k == 0 && !e.level().isClientSide() && isMergable(e, stack)) {
            mergeWithNeighbours(e, stack);
        }
        // index cell update — ровно на floor-change тиках (moved == floor-change)
        if (moved && !e.level().isClientSide()) {
            int[] box = idMap.get(e);
            if (box != null) {
                int rc = idxSetCell(box[0], System.identityHashCode(e.level()),
                        Mth.floor(e.getX()), Mth.floor(e.getY()), Mth.floor(e.getZ()));
                if (rc != 0) {
                    indexBroken = true;
                }
            }
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

    /**
     * Реплика vanilla mergeWithNeighbours (javap [0..168], см. RESEARCH-J §1)
     * с кандидатами из собственного rust-индекса вместо
     * level.getEntitiesOfClass. Тела tryToMerge/walls-fix/break-on-removed —
     * ванильные. Отказ индекса → ванильный merge (MethodHandle).
     */
    private static void mergeWithNeighbours(ItemEntity self, ItemStack selfStack) {
        if (!isMergable(self, selfStack) || indexBroken) {
            if (isMergable(self, selfStack)) {
                invokeVanillaMerge(self);
            }
            return;
        }
        int[] box = idMap.get(self);
        if (box == null) {
            // Ленивая индексация: item, добавленный до армирования/без хука,
            // попадает в индекс на первом merge-gate тике (idempotent).
            indexAdd(self);
            box = idMap.get(self);
            if (box == null) {
                invokeVanillaMerge(self); // !READY / broken — точный vanilla
                return;
            }
        }
        Level lvl = self.level();
        double r = lvl.spigotConfig.itemMerge;
        AABB qb = self.getBoundingBox().inflate(r,
                lvl.paperConfig().entities.behavior.onlyMergeItemsHorizontally ? 0.0D : r - 0.5D, r);
        int lid = System.identityHashCode(lvl);
        int[] out = SCRATCH.get();
        int n = idxQuery(qb.minX, qb.minY, qb.minZ, qb.maxX, qb.maxY, qb.maxZ, lid, out);
        if (n < 0) {
            if (n <= -2) {
                out = new int[(-n) * 4];
                SCRATCH.set(out);
                n = idxQuery(qb.minX, qb.minY, qb.minZ, qb.maxX, qb.maxY, qb.maxZ, lid, out);
            }
            if (n < 0) {
                indexBroken = n == -1;
                invokeVanillaMerge(self);
                return;
            }
        }
        boolean walls = lvl.paperConfig().fixes.fixItemsMergingThroughWalls;
        for (int i = 0; i < n; i++) {
            int cid = out[i];
            // guard: индекс может вырасти параллельно (lazi indexAdd с соседнего
            // воркера) — читаем актуальный массив с bounds-check
            ItemEntity[] ids = byId;
            if (cid < 0 || cid >= ids.length) {
                continue;
            }
            ItemEntity other = ids[cid];
            // ванильный предикат lambda$mergeWithNeighbours$0 + точность запроса;
            // isMergable приватен — реплика (javap-точно), getItem() public
            if (other == null || other == self || other.level() != lvl
                    || !isMergable(other, other.getItem())) {
                continue;
            }
            if (!other.getBoundingBox().intersects(qb)) {
                continue;
            }
            if (walls && lvl.clipDirect(self.position(), other.position(),
                    CollisionContext.of(self)) == HitResult.Type.BLOCK) {
                continue;
            }
            tryToMerge(self, other);
            if (self.isRemoved()) {
                return;
            }
        }
    }

    private static void tryToMerge(ItemEntity self, ItemEntity other) {
        try {
            MH_TRY_TO_MERGE.invokeExact(self, other);
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
}
