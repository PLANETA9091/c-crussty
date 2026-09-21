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

    /** foot2 (TASK-399-J): J-гейт расширен compose-контрактом round-399 —
     *  собственный флаг cmp399_foot2 (любой cmp399_*) армит ту же подсистему
     *  на базе round-398-j-subsys2. */
    private static final boolean ENABLED = flagMatches(trimToEmpty(System.getenv("CRUSSTY_LEVER_FLAG")));

    private static boolean flagMatches(String f) {
        return "items_subsys2".equals(f) || f.startsWith("cmp399_");
    }

    /** foot2 site-census (см. RESEARCH-J §2): flatten/hoist/reorder-точки. */
    static final int FOOT2_SITES = 15;

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

    /** foot2 site#1: телеметрия — per-thread счётчик вместо static long
     *  (++telemetryCounter из 4 region-воркеров = false-sharing RFO-шторм на
     *  одну кэш-линию ~3M коherency-оп/с при 150k сущностей). Ноль аллокаций
     *  в steady-state, каденс лога — диагностика, паритет не затронут. */
    private static final ThreadLocal<long[]> TELEMETRY_TL =
            ThreadLocal.withInitial(() -> new long[1]);

    private static final int TELEMETRY_INTERVAL = 24000;

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
        long calls = ++TELEMETRY_TL.get()[0];
        if (calls % TELEMETRY_INTERVAL == 0L) {
            LOG.info("[crussty-plugin] items_subsys2: telemetry calls=" + calls
                    + " (foot2: per-thread, no false sharing)");
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
        // foot2 sites#2/#3: level+clientSide — по одному чтению на тик
        // (было 6+ виртуальных e.level() / 4 isClientSide на том же поле).
        Level lvl = e.level();
        boolean clientSide = lvl.isClientSide();
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
        // foot2 site#4: AABB hoist — одна getBoundingBox-цепочка вместо двух
        // (deflate создаёт НОВЫЙ AABB, self.bb не мутируется → ссылка валидна).
        if (clientSide) {
            e.noPhysics = false;
        } else {
            AABB bb = e.getBoundingBox();
            e.noPhysics = !lvl.noCollision(e, bb.deflate(1.0E-7D));
            if (e.noPhysics) {
                e.moveTowardsClosestSpace(e.getX(), (bb.minY + bb.maxY) / 2.0D, e.getZ());
            }
        }
        // offsets 226..270: move-гейт
        // foot2 site#5: один getDeltaMovement вместо двух (между чтением
        // условия и move() писателей dm нет — условие чистое).
        Vec3 dm = e.getDeltaMovement();
        if (e.onGround() && dm.horizontalDistanceSqr() <= 9.999999747378752E-6D
                && (e.tickCount + e.getId()) % 4 != 0) {
            // skip move (vanilla branch)
        } else {
            e.move(MoverType.SELF, dm);
        }
        // offset 272
        e.applyEffectsFromBlocks();
        // offsets 276..341: friction
        float f = 0.98F;
        if (e.frictionState == net.kyori.adventure.util.TriState.FALSE) {
            f = 1.0F;
        } else if (e.onGround()) {
            f = lvl.getBlockState(e.getBlockPosBelowThatAffectsMyMovement()).getBlock()
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
        // foot2 sites#6/#7/#8: Mth.floor×6 → инлайн floorI (javap-точно:
        // d2i; dcmpg; ifge) + скалярные локалы px/py/pz (3 чтения позиции
        // вместо 6); fx/fy/fz переиспользуются в idxSetCell — ваниль писала
        // их заново ПОСЛЕ merge-окна, но tryToMerge/merge НЕ двигают self
        // (javap: только stack/age/pickupDelay/discard) → значения те же.
        double px = e.getX();
        double py = e.getY();
        double pz = e.getZ();
        int fx = floorI(px);
        int fy = floorI(py);
        int fz = floorI(pz);
        int fxo = floorI(e.xo);
        int fyo = floorI(e.yo);
        int fzo = floorI(e.zo);
        boolean moved = fxo != fx || fyo != fy || fzo != fz;
        int k = moved ? 2 : 40;
        // foot2 site#9: despawnRate — один MH-геттер на тик (гейт + despawn),
        // поле приватное per-item, писателей в тике нет.
        int despawnRate = getDespawnRate(e);
        if (e.tickCount % k == 0 && !clientSide && isMergable(e, stack, despawnRate)) {
            mergeWithNeighbours(e, stack);
        }
        // index cell update — ровно на floor-change тиках (moved == floor-change)
        if (moved && !clientSide) {
            int[] box = idMap.get(e);
            if (box != null) {
                int rc = idxSetCell(box[0], System.identityHashCode(lvl), fx, fy, fz);
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
        // foot2 site#10: без аллокации Vec3.subtract+lengthSqr — та же
        // fp-математика поразрядно (subtract: x1-x2; lengthSqr: x*x+y*y+z*z).
        if (!clientSide) {
            Vec3 dmNow = e.getDeltaMovement();
            double ddx = dmNow.x - vec3.x;
            double ddy = dmNow.y - vec3.y;
            double ddz = dmNow.z - vec3.z;
            if (ddx * ddx + ddy * ddy + ddz * ddz > 0.01D) {
                e.hasImpulse = true;
            }
        }
        // offsets 544..588: despawn
        if (!clientSide && e.age >= despawnRate) {
            if (CraftEventFactory.callItemDespawnEvent(e).isCancelled()) {
                e.age = 0;
                return;
            }
            e.discard(EntityRemoveEvent.Cause.DESPAWN);
        }
    }

    /** Побайтная инлайн-реплика Mth.floor(double) (javap round-j2b: d2i;
     *  dcmpg; ifge; isub) — убирает статический вызов из moved-чека. */
    private static int floorI(double v) {
        int i = (int) v;
        return v < (double) i ? i - 1 : i;
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
        return isMergable(e, stack, getDespawnRate(e));
    }

    /** foot2 site#9: вариант с уже поднятым despawnRate (self-путь tickBody). */
    private static boolean isMergable(ItemEntity e, ItemStack stack, int despawnRate) {
        return e.isAlive()
                && e.pickupDelay != 32767
                && e.age != -32768
                && e.age < despawnRate
                && stack.getCount() < stack.getMaxStackSize();
    }

    /**
     * Реплика vanilla mergeWithNeighbours (javap [0..168], см. RESEARCH-J §1)
     * с кандидатами из собственного rust-индекса вместо
     * level.getEntitiesOfClass. Тела tryToMerge/walls-fix/break-on-removed —
     * ванильные. Отказ индекса → ванильный merge (MethodHandle).
     */
    private static void mergeWithNeighbours(ItemEntity self, ItemStack selfStack) {
        // foot2 site#11: isMergable — ОДИН вычисленный локал (было: условие
        // !isMergable||broken + повторный вызов внутри = 2 MH-цепочки на entry).
        // Таблица исходов идентична: !mg → no-op (ваниль тоже no-op при
        // !isMergable); mg&&broken → vanilla; !mg&&broken → no-op.
        boolean mg = isMergable(self, selfStack);
        if (!mg) {
            return;
        }
        if (indexBroken) {
            invokeVanillaMerge(self);
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
        // foot2 site#12: selfBb hoist — один виртуальный геттер вместо двух чтений.
        AABB selfBb = self.getBoundingBox();
        AABB qb = selfBb.inflate(r,
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
        // foot2 sites#13/#14: byId+cap — ДО цикла (было чтение static-поля
        // на КАЖДОГО кандидата — dependence-chain в цикле); фильтр intersects
        // (чистые field-reads) ДО isMergable (MH+stack) — предикаты чистые,
        // множество дошедших до tryToMerge идентично.
        ItemEntity[] ids = byId;
        int cap = ids.length;
        // ПОРЯДОК ОБХОДА СОХРАНЁН 1:1 (rust-chain порядок = семантика):
        // javap tryToMerge — other.count >= self.count поглощает SELF, порядок
        // определяет выжившего ⇒ сортировка/реверс кандидатов ЗАПРЕЩЕНЫ.
        for (int i = 0; i < n; i++) {
            int cid = out[i];
            if (cid < 0 || cid >= cap) {
                continue;
            }
            ItemEntity other = ids[cid];
            if (other == null || other == self || other.level() != lvl) {
                continue;
            }
            if (!other.getBoundingBox().intersects(qb)) {
                continue;
            }
            // isMergable приватен — реплика (javap-точно), getItem() public
            if (!isMergable(other, other.getItem())) {
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
