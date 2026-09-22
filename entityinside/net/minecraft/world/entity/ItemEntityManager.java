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
 * TASK-399-F (vector despawnv2, flag cmp399_despawn2): despawn-хвост item-фазы
 * ведётся RUST lifetime-heap (src/items_lifetime.rs) — per-item despawn-гейт
 * (реплика offsets 544..588) из tickBody уходит, дедлайны пушатся батчем
 * (один native/тик), drain — один native/тик, применение — ВАНИЛЬНЫЙ flow
 * (callItemDespawnEvent → cancel? age=0 : discard(DESPAWN)) на each due-id.
 * События не пропускаются; java.util-куча не используется вовсе.
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

    private static final String LEVER_FLAG = trimToEmpty(System.getenv("CRUSSTY_LEVER_FLAG"));

    /**
     * TASK-399-B (cmp399_shard): gate = legacy flag OR the whole cmp399_* lever
     * family (family-gate: J-подсистема армится любым cmp399_*-флагом).
     * TASK-400-A (cmp399_bfcomp composite): the committed build/…class is
     * recompiled from THIS merged source, so ENABLED and DESPAWN2 below bake
     * the composite flag in directly (no runtime CP-patch needed for it — the
     * rust side patches only the legacy cmp399_shard path, byte-parity A/B).
     */
    private static final boolean ENABLED =
            "items_subsys2".equals(LEVER_FLAG) || LEVER_FLAG.startsWith("cmp399_")
                    // TASK-402-B: главный композит раунда — shardgrid+heap
                    // включаются наряду с мобовыми soa+grid единым флагом.
                    || "cmp402_comp".equals(LEVER_FLAG)
                    // TASK-402-F: stagcomp = композит + stagger (единый флаг).
                    || "cmp402_stagcomp".equals(LEVER_FLAG)
                    // TASK-403-C: tickplane whole-body plane (items-plane
                    // сегмент; STRICT eq, пустой флаг = ваниль).
                    || "cmp403_tickplane".equals(LEVER_FLAG)
                    // TASK-405-F: композит stagcomp⊕tickplane.
                    || "cmp405_stagtick".equals(LEVER_FLAG)
                    // TASK-406-D: композит раунда-406 (stagtick ⊕ ai-window).
                    || "cmp406_aibatch".equals(LEVER_FLAG)
                    // TASK-406-E: композит раунда-406 (stagtick ⊕ sscan).
                    || "cmp406_sscan".equals(LEVER_FLAG)
                    // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                    || "cmp409_multi".equals(LEVER_FLAG) || "cmp412_meganav".equals(LEVER_FLAG);

    /**
     * TASK-403-C2 ITEM-PLANE RESTING (точный флаг cmp403_tickplane, STRICT eq;
     * пустой/чужой флаг = ваниль бит-в-бит).
     *
     * Root-cause (lane_map round403cl1/cl2, пары к базлайну r402anc1):
     * fluid-лейн 17-19% (~19-22k сэмплов) — это ItemEntityManager.tickBody →
     * Entity.baseTick → updateInWaterStateAndDoFluidPushing: item-плейн гоняет
     * ПОЛНЫЙ faithful tickBody (fluid-push ×2 + inside + move + friction-block
     * + merge-scan) на КАЖДЫЙ тик каждого из ~150k item, хотя большинство —
     * resting (onGround, hdSqr <= 1e-5 — ванильный move-gate predicate).
     * ITEM-PLANE state machine (дизайн TASK-402-E, upstream-пруфы: Paper
     * ActivationRange inactiveTick / Pufferfish DAB / LOD-ticking) НЕ была
     * реализована в базе 3c03dad (stage=setup).
     *
     * МЕХАНИКА: resting item тикает фазой (tickCount+id)&31==0 полным faithful
     * tickBody (support/fluid recheck 1/32), в остальные 31/32 тиков —
     * e.inactiveTick(): ПОБАЙТОВАЯ upstream-семантика Paper ActivationRange
     * (javap pruf: Entity.inactiveTick no-op + pickupDelay-- guard + age++ +
     * despawn-gate с CraftEventFactory.callItemDespawnEvent) + merge-gate на
     * ванильном cadence tickCount%40==0 (k=40 resting, javap offsets 376..473).
     *
     * ВАНИЛЬНОСТЬ: despawn-тайминг точный (age++ и despawn-gate каждый тик,
     * один ItemDespawnEvent — heap stale-early см. lifetimeDue); merge —
     * только ванильно-eligible тики; вода/лава/портал/свежий дроп (pickupDelay
     * >0) = ACTIVE per-tick faithful; переходы resting→active по fluid recheck
     * 1/32 (<=32t transient, строже upstream inactiveTick, где recheck
     * отсутствует вовсе).
     */
    private static final boolean REST_PLANE =
            "cmp403_tickplane".equals(LEVER_FLAG) || "cmp405_stagtick".equals(LEVER_FLAG)
            // TASK-406-D: композит раунда-406 включает item-plane resting.
            || "cmp406_aibatch".equals(LEVER_FLAG)
            // TASK-406-E: композит раунда-406 (stagtick ⊕ sscan).
            || "cmp406_sscan".equals(LEVER_FLAG)
            // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
            || "cmp409_multi".equals(LEVER_FLAG) || "cmp412_meganav".equals(LEVER_FLAG);

    /** TASK-399-F despawnv2: rust lifetime-heap + батч-деспавн (точный флаг).
     *  TASK-400-A: составной флаг cmp399_bfcomp (B+F) включает despawnv2
     *  наряду с точным cmp399_despawn2 — векторы ортогональны
     *  (read-scaling vs despawn-хвост) и армятся одновременно. */
    private static final boolean DESPAWN2 =
            "cmp399_despawn2".equals(LEVER_FLAG) || "cmp399_bfcomp".equals(LEVER_FLAG)
                    // TASK-402-B: композит включает lifetime-heap суб-механизм.
                    || "cmp402_comp".equals(LEVER_FLAG)
                    || "cmp402_stagcomp".equals(LEVER_FLAG)
                    // TASK-403-C: tickplane включает lifetime-heap сегмент.
                    || "cmp403_tickplane".equals(LEVER_FLAG)
                    // TASK-405-F: композит stagcomp⊕tickplane.
                    || "cmp405_stagtick".equals(LEVER_FLAG)
                    // TASK-406-D: композит раунда-406 (stagtick ⊕ ai-window).
                    || "cmp406_aibatch".equals(LEVER_FLAG)
                    // TASK-406-E: композит раунда-406 (stagtick ⊕ sscan).
                    || "cmp406_sscan".equals(LEVER_FLAG)
                    // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                    || "cmp409_multi".equals(LEVER_FLAG) || "cmp412_meganav".equals(LEVER_FLAG);

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

    // ---- natives (impl: src/items_lifetime.rs, TASK-399-F despawnv2) ----
    /** Батч-пуш дедлайнов: long = id<<32 | due&0xFFFFFFFF; возвращает 0/err. */
    private static native int lifetimePush(long[] batch, int n);
    /** Drain всех due <= nowTick; -total для grow-retry (записи ждут в staging). */
    private static native int lifetimeDue(long nowTick, long[] out);

    /** despawnv2 активен (mode=true, нативы живы, индекс не сломан). Читается
     *  воркерами каждый item-тик; пишется main-потоком между фазами. */
    private static volatile boolean despawn2Active;
    /** mode: lifetime-хук активирован (enmass уже сделан, поллы идут). */
    private static boolean despawn2Mode;

    /** Буфер indexAdd-ов текущего тика (ids) — flush одним lifetimePush. */
    private static int[] pushBuf = new int[256];
    private static int pushTop = 0;
    private static final Object PUSH_LOCK = new Object();
    /** Drain-scratch: grow-only, ноль аллокаций в steady-state. */
    private static long[] dueScratch = new long[4096];

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
        if (REST_PLANE) {
            // громкий ARM-маркер новой нога-механики (server-stdout.log pruf)
            LOG.info("[crussty-plugin] items_restplane ARMED phase=32 resting=inactiveTick"
                    + " merge_gate=vanilla%40 fluid_recheck=1/32");
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
    // TASK-399-F despawnv2: rust lifetime-heap + batch despawn
    // ------------------------------------------------------------------

    /** Буферизовать дедлайн-пуш для id (вычисляется на flush от live-age).
     *  Вызывается из indexAdd (в т.ч. с воркера — lazy indexAdd). */
    private static void lifetimeBufferPush(int id) {
        if (!DESPAWN2 || !despawn2Active) {
            return;
        }
        synchronized (PUSH_LOCK) {
            if (pushTop == pushBuf.length) {
                pushBuf = java.util.Arrays.copyOf(pushBuf, pushTop * 2);
            }
            pushBuf[pushTop++] = id;
        }
    }

    /** Абсолютный дедлайн записи конца тика: due = now + max(0, rate - age).
     *  Вывод (RESEARCH-F): запись конца тика T с возрастом A срабатывает
     *  ванильно в конце T+j ⟺ A+j >= rate ⟺ due = T + (rate - A). */
    private static long lifetimeDueOf(ItemEntity e, long now) {
        int remaining = getDespawnRate(e) - e.age;
        if (remaining < 0) {
            remaining = 0;
        }
        return now + remaining;
    }

    /** Активация despawnv2: enmass-пуш всех проиндексированных items (один
     *  батч), далее — только diff за тик. main-поток, после join фазы. */
    private static void lifetimeEnmass(long now) {
        long[] batch = new long[idMap.size()];
        int n = 0;
        for (java.util.Map.Entry<ItemEntity, int[]> en : idMap.entrySet()) {
            ItemEntity e = en.getKey();
            int id = en.getValue()[0];
            if (e.isRemoved() || e.age == -32768 || e.level().isClientSide()) {
                continue;
            }
            batch[n++] = packLifetime(id, lifetimeDueOf(e, now));
        }
        if (n > 0) {
            lifetimePush(batch, n);
        }
        LOG.info("[crussty-plugin] " + LEVER_FLAG + ": lifetime-heap active (enmass=" + n + ")");
    }

    private static long packLifetime(int id, long due) {
        return ((long) id << 32) | (due & 0xFFFFFFFFL);
    }

    /** Flush буфера indexAdd-ов: due считается от КОНЦА текущего тика (age
     *  уже финальный: потикавшиеся — post-increment, PENDING-add — pre-first-
     *  tick; обе ветки покрываются одной формулой lifetimeDueOf). */
    private static void lifetimeFlush(long now) {
        int n;
        int[] ids;
        synchronized (PUSH_LOCK) {
            n = pushTop;
            pushTop = 0;
            ids = pushBuf;
        }
        if (n <= 0) {
            return;
        }
        long[] batch = new long[n];
        int m = 0;
        ItemEntity[] table = byId;
        for (int i = 0; i < n; i++) {
            int id = ids[i];
            if (id < 0 || id >= table.length) {
                continue;
            }
            ItemEntity e = table[id];
            if (e == null || e.isRemoved() || e.age == -32768 || e.level().isClientSide()) {
                continue;
            }
            batch[m++] = packLifetime(id, lifetimeDueOf(e, now));
        }
        if (m > 0) {
            lifetimePush(batch, m);
        }
    }

    /** Прямой точечный пуш (cancel/stale-early re-push; вне буфера — буфер
     *  уже флашен на этом тике). */
    private static void lifetimePushOne(int id, long due) {
        lifetimePush(new long[] {packLifetime(id, due)}, 1);
    }

    /** Per-tick despawnv2-хук: RegionTickOps.forEach, main, после join фазы
     *  (PENDING drain уже обработал indexAdd/indexRemove). Один native-push
     *  + один native-drain на тик; применение — ванильный flow. */
    public static void lifetimeTick() {
        if (!DESPAWN2 || despawn2ModeBroken()) {
            despawn2Active = false;
            return;
        }
        try {
            long now = net.minecraft.server.MinecraftServer.currentTick;
            if (!despawn2Mode) {
                despawn2Mode = true;
                lifetimeEnmass(now);
            }
            lifetimeFlush(now);
            lifetimePoll(now);
            despawn2Active = !indexBroken;
        } catch (Throwable t) {
            // fail-closed: ванильная despawn-ветка tickBody вернётся; записи
            // кучи подчищаются последующими поллами/верификацией.
            despawn2Active = false;
            LOG.severe("[crussty-plugin] " + LEVER_FLAG + ": lifetime tick failed — vanilla despawn branch restored: " + t);
        }
    }

    private static boolean despawn2ModeBroken() {
        return !READY || indexBroken || !probeOnce();
    }

    /** Полл due-id и применение ВАНИЛЬНОГО despawn-flow. */
    private static void lifetimePoll(long now) {
        long[] due = dueScratch;
        int n = lifetimeDue(now, due);
        if (n < 0) {
            due = new long[-n];
            dueScratch = due;
            n = lifetimeDue(now, due);
        }
        if (n < 0) {
            indexBroken = true; // структурный отказ — весь путь в vanilla
            return;
        }
        for (int i = 0; i < n; i++) {
            long l = due[i];
            int id = (int) (l >> 32);
            ItemEntity[] table = byId;
            if (id < 0 || id >= table.length) {
                continue;
            }
            ItemEntity e = table[id];
            if (e == null || e.isRemoved()) {
                continue; // stale запись (merge/pickup/discard раньше due)
            }
            if (e.level().isClientSide()) {
                continue;
            }
            int rate = getDespawnRate(e);
            if (e.age < rate) {
                // stale-early: merge делает survivor.age = min(...) — дедлайн
                // пересчитать от live-возраста (событие НЕ вызываем).
                lifetimePushOne(id, now + Math.max(0, rate - e.age));
                continue;
            }
            // ВАНИЛЬНЫЙ flow (javap tick()V 565..585): event → cancel? age=0 : discard
            if (CraftEventFactory.callItemDespawnEvent(e).isCancelled()) {
                e.age = 0;
                lifetimePushOne(id, now + Math.max(0, rate));
            } else {
                e.discard(EntityRemoveEvent.Cause.DESPAWN);
            }
        }
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
            lifetimeBufferPush(id); // despawnv2: дедлайн считается на flush конца тика
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
            if (REST_PLANE && planeResting(e)) {
                if (((e.tickCount + e.getId()) & 31) == 0) {
                    // plane tick: полный faithful tickBody — support/fluid
                    // recheck 1/32 (переходы resting→active, buoyancy onset)
                    tickBody(e);
                    e.postTick();
                } else {
                    // upstream Paper ActivationRange resting-семантика:
                    // pickupDelay-- guard + age++ + despawn-gate (javap pruf),
                    // ноль voxel-сканов baseTick
                    e.inactiveTick();
                    // merge-gate на ванильном resting cadence (k=40): скан
                    // только в vanilla-eligible тики, кандидаты из индекса
                    if (e.tickCount % 40 == 0 && !e.level().isClientSide()) {
                        ItemStack stack = e.getItem();
                        if (isMergable(e, stack)) {
                            mergeWithNeighbours(e, stack);
                        }
                    }
                }
            } else {
                tickBody(e);
                e.postTick();
            }
        } else {
            e.inactiveTick();
        }
    }

    /**
     * Ванильный resting-предикат (move-gate offsets 226..270 minus %4-phase):
     * onGround + hdSqr <= 9.999999747378752E-6. Консервативные исключения —
     * всё, что требует per-tick faithful семантики: fluid (вода/лава —
     * buoyancy), свежий дроп (pickupDelay > 0 && != 32767), сущность в портале
     * (postTick.handlePortal — public field, дешёвое чтение).
     */
    private static boolean planeResting(ItemEntity e) {
        if (!e.onGround() || e.portalProcess != null) {
            return false;
        }
        int pd = e.pickupDelay;
        if (pd != 0 && pd != 32767) {
            return false;
        }
        return !e.isInWater() && !e.isInLava()
                && e.getDeltaMovement().horizontalDistanceSqr() <= 9.999999747378752E-6D;
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
        // offsets 544..588: despawn — vanilla; при despawnv2 гейт уходит в
        // rust lifetime-heap (lifetimeTick после фазы: тот же тик, ванильный
        // flow, события не пропускаются; age++ выше остаётся per-item).
        if (!despawn2Active && !e.level().isClientSide() && e.age >= getDespawnRate(e)) {
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
