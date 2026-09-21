package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.Predicate;

import ca.spottedleaf.moonrise.common.PlatformHooks;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

/**
 * MOB-SOA (TASK-401-E, vector soa — lever cmp401_soa).
 *
 * STRUCTURE-OF-ARRAYS flat mirror broadphase для мобового push-лейна (DOD
 * реврайт data-плоскости round-400-J mobs_grid: вместо per-slot seqlock
 * записей — плоские параллельные векторы x/y/z/hw/hh/flags по плотному id).
 * Порт семантики апстримного Paper «optimize entity pushes»
 * (paper-server/patches/sources/net/minecraft/world/entity/LivingEntity.java.patch:
 * «don't run getEntities if we're not going to use its result» + «Cap entity
 * collisions»): ванильное тело LivingEntity.pushEntities потребляет результат
 * Level.getPushableEntities(this, this.getBoundingBox()) ТОЛЬКО циклом
 * doPush(entity) + cramming-счётчиками. Этот бридж заменяет ТОЛЬКО
 * перечисление кандидатов:
 *
 *   - ГОРЯЧИЕ ПОЛЯ пушатся в rust SoA-плоскость per-tick (mobUpsert: id +
 *     lid + центр AABB + hw/hh радиусы; bounding-радиус-гейт r_eff =
 *     max(halfW, halfH) <= 1.0 java-стороной — в бенч-популяции все мобы
 *     <= 0.7/2.0), без объектов — только плотный id->Entity массив
 *     java-стороны; rust держит ПЛОСКИЕ массивы (structure-of-arrays,
 *     фиксированная ёмкость, без реаллокаций и указателей);
 *   - push-кандидаты отдаются java ОДНИМ запросом mobQuery(box, lid, out):
 *     SoA-скан = одна глобальная seqlock-версия на запрос (не per-cell
 *     v1->данные->v2), обход цепочек 1.0-ячеек с ПЛОСКИМ чтением полей и
 *     грубым center±радиусы AABB-пруном прямо в rust (суперсет
 *     AABB.intersects: hw >= истинных x/z полуэкстентов — доказ. в
 *     mobs_soa.rs); java фильтрует только истинных соседей;
 *   - точные ванильные фильтры на каждом кандидате: other.level() == level,
 *     AABB.intersects(box), EntitySelector.pushableBy(entity) (предикат
 *     ванильный — Scoreboard-лейн сохраняется), other != entity;
 *   - хвост LivingEntity.pushEntities (cramming + numCollisions + doPush) —
 *     НЕ ТРОНУТ ванильный байткод; смещения одного пуша идентичны ванильным
 *     по построению, порядок кандидатов = порядок цепочек ячеек (документир.
 *     дельта класса items_subsys2).
 *
 * СВЕЖЕСТЬ (без epsilon): pushEntities вызывается в aiStep ПОСЛЕ travel/move
 * (javap offsets 620..850), поэтому само-апсерт в момент запроса кладёт в
 * плоскость end-of-move позицию self; кандидаты, тикающие позже в этом же
 * тике, ещё не двигались (их SoA-позиция = их прошлый end-of-tick =
 * текущая). Взаимный push(Entity) меняет только deltaMovement — позиция
 * интегрируется собственным move() пушуемого (Paper Entity.java.patch
 * push(Entity): additive deltas) ⇒ living<->living свежесть точная в
 * пределах однопроходного тика.
 *
 * FAIL-CLOSED: ENABLED (env == "cmp401_soa", STRICT eq) && nativeOk (mobProbe
 * magic) && !broken (структурный отказ плоскости — дизарм навсегда) &&
 * !oversized (в популяции замечен r_eff > 1.0 — весь рычаг в ванильный
 * режим) — иначе 100% ванильный fill (точная реплика fill-последовательности
 * Level.getEntities: Profiler-счётчик + EntityLookup.getEntities +
 * PlatformHooks.addToGetEntities, EntityQueryOps-контракт). Любой ненулевой
 * rc mobUpsert → ваниль на ЭТОТ вызов (self не опубликован — грид не
 * используется), ERR_STRUCT — дизарм.
 * Пустой/чужой CRUSSTY_LEVER_FLAG — сайт вообще не ретаргетится (rust-сторона
 * не ставит патч), путь ванильный по построению.
 */
public final class MobPushOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // TASK-402-B: the round-402 composite arms the SoA plane (primary)
        // together with the mobs_grid sharded mirror (per-call fallback read
        // plane — mobGridQuery below). Legacy cmp401_soa keeps its exact
        // prior behavior: grid natives are never called under it.
        return f != null
                && (f.trim().equals("cmp401_soa") || f.trim().equals("cmp402_comp")
                    || f.trim().equals("cmp402_stagcomp")
                    // TASK-403-B: jnibulk = композит + bulk-транспорт; мобовая
                    // SoA-плоскость армится тем же флагом (иначе PARTIAL-ARM —
                    // урок 60fe902).
                    || f.trim().equals("cmp403_jnibulk"));
    }

    private static final boolean ENABLED = leverEnabled();

    /** TASK-402-B: composite mode (mirror-grid fallback active). */
    private static boolean compositeEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && (f.trim().equals("cmp402_comp")
                || f.trim().equals("cmp402_stagcomp")
                // TASK-403-B: зеркальный grid fallback — часть композита.
                || f.trim().equals("cmp403_jnibulk"));
    }

    private static final boolean COMPOSITE = compositeEnabled();

    /** TASK-404-B: fused mob-push step gate (STRICT eq — only the jnibulk
     *  bulk-transport flag; legacy flags keep the exact two-transition leg1
     *  path; empty/foreign flag never reaches the fused native). */
    private static boolean fusedEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp403_jnibulk");
    }

    private static final boolean FUSED = fusedEnabled();
    /** One-shot effect-proof маркер первого успешного fused-вызова (stdout). */
    private static volatile boolean fusedProof;

    private static final int PROBE_MAGIC = 0x5053; // "SOA"
    private static final int GRID_PROBE_MAGIC = 0x4D50; // "MP" (mobs_grid)

    /** Result codes natives: >=0 ok/count; -1 ERR_STRUCT (дизарм); -2 ERR_RANGE (per-call vanilla); -(cap) overflow. */
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    /** Overflow-порог: -(cap) при стартовом scratch 256 -> n <= -256; -2 не пересекается. */
    private static final int OVERFLOW_MAX = -256;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/mobs_soa.rs, RegisterNatives после define) ----
    private static native int mobProbe();
    private static native int mobUpsert(int id, int lid, double x, double y, double z,
            double hw, double hh);
    private static native int mobRemove(int id);
    private static native int mobQuery(double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int lid, int[] out);

    // ---- TASK-402-B composite: sharded mirror grid (impl: src/mobs_grid.rs;
    // кандидаты — те же плотные id SoA-плоскости, резолвятся тем же byId) ----
    private static native int mobGridProbe();
    private static native int mobGridQuery(double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int lid, int[] out);

    // ---- TASK-404-B jnibulk deepening: FUSED self-upsert + candidate scan
    // в ОДНОМ JNI-переходе (impl: src/mobs_soa.rs mob_push_step; семантика =
    // mobUpsert(...) затем mobQuery(...) в одной WLOCK-критической секции,
    // single-pass скан без seqlock-ретраев; overflow -(cap), ERR-лестница 1:1
    // с leg1). STRICT: активен только под cmp403_jnibulk — legacy-флаги идут
    // точным leg1-путём двух переходов; пустой/чужой флаг = не вызывается.
    private static native int mobPushStep(int id, int lid,
            double x, double y, double z, double hw, double hh,
            double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int[] out);

    private static volatile boolean nativeOk;
    private static volatile boolean gridNativeOk;
    private static volatile boolean broken;
    private static volatile boolean oversized;

    /** id -> entity (плотный массив, grow x2; ids реиспользуются через freeIds). */
    private static Entity[] byId = new Entity[1024];
    private static int idTop = 0;
    private static int[] freeIds = new int[256];
    private static int freeTop = 0;
    /** entity -> id-box. Пишется под ID_LOCK; читается воркерами. */
    private static final ConcurrentHashMap<Entity, int[]> idMap = new ConcurrentHashMap<>();
    private static final Object ID_LOCK = new Object();

    /** Query scratch: per-thread, grow-only, ноль аллокаций в steady-state. */
    private static final ThreadLocal<int[]> SCRATCH =
            ThreadLocal.withInitial(() -> new int[256]);

    /** Кольцо результатов (EntityQueryOps-паттерн: 8 слотов, вложенные запросы до глубины 7). */
    private static final int RING_SLOTS = 8;
    private static final ThreadLocal<ArrayList<Entity>[]> RING =
            ThreadLocal.withInitial(MobPushOps::newRing);
    private static final ThreadLocal<int[]> RING_CURSOR =
            ThreadLocal.withInitial(() -> new int[1]);

    @SuppressWarnings("unchecked")
    private static ArrayList<Entity>[] newRing() {
        ArrayList<Entity>[] ring = new ArrayList[RING_SLOTS];
        for (int i = 0; i < RING_SLOTS; i++) {
            ring[i] = new ArrayList<>(16);
        }
        return ring;
    }

    private MobPushOps() {}

    /** Ленивая проверка нативов (первый armed(); до регистрации — Throwable -> false, ретрай). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (MobPushOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = mobProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Ленивая проверка зеркального грида (только под композитом). */
    private static boolean gridProbeOnce() {
        if (!COMPOSITE || broken) {
            return false;
        }
        if (gridNativeOk) {
            return true;
        }
        synchronized (MobPushOps.class) {
            if (gridNativeOk) {
                return true;
            }
            try {
                gridNativeOk = mobGridProbe() == GRID_PROBE_MAGIC;
            } catch (Throwable t) {
                gridNativeOk = false;
            }
            return gridNativeOk;
        }
    }

    /** Gate для rust-стороны/диагностики: армирован ли бридж. */
    public static boolean armed() {
        return ENABLED && !broken && !oversized && probeOnce();
    }

    /**
     * TASK-404-B (jnibulk deepening): FUSED steady-state путь — mobPushStep
     * (upsert+scan = 1 JNI-переход). @return null — первый вызов сущности
     * (id ещё не аллоцирован): вызывающий уходит в legacy-путь (alloc +
     * publish ровно как в leg1); иначе — результат (включая per-call vanilla
     * лестницы ошибок, 1:1 с leg1: upsert-ERR_RANGE = ваниль на вызов БЕЗ
     * grid-read, ERR_STRUCT = дизарм, overflow = grow ×4 → retry fused →
     * grid-fallback → vanilla).
     */
    private static List<Entity> pushablesFused(Level level, Entity entity, AABB box) {
        int[] boxId = idMap.get(entity);
        if (boxId == null) {
            return null; // первый вызов сущности — legacy alloc-путь
        }
        AABB bb = entity.getBoundingBox();
        double hw = Math.max((bb.maxX - bb.minX) * 0.5D, (bb.maxZ - bb.minZ) * 0.5D);
        double hh = (bb.maxY - bb.minY) * 0.5D;
        if (Math.max(hw, hh) > 1.0D) {
            if (!oversized) {
                oversized = true; // не-грид-юниверс: весь рычаг в ваниль (fail-closed)
                LOG.warning("[crussty-plugin] cmp401_soa: oversized bounding radius "
                        + Math.max(hw, hh) + " on " + entity.getType() + " — lever reverted to vanilla");
            }
            return vanillaFill(level, entity, box);
        }
        double cx = (bb.minX + bb.maxX) * 0.5D;
        double cy = (bb.minY + bb.maxY) * 0.5D;
        double cz = (bb.minZ + bb.maxZ) * 0.5D;
        int lid = System.identityHashCode(level);
        int[] out = SCRATCH.get();
        int n = mobPushStep(boxId[0], lid, cx, cy, cz, hw, hh,
                box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ, out);
        if (n >= 0) {
            fusedProofOnce();
            return collect(level, entity, box, out, n);
        }
        if (n == ERR_STRUCT) {
            broken = true; // структурный отказ — весь рычаг дизармится
            return vanillaFill(level, entity, box);
        }
        if (n <= OVERFLOW_MAX) {
            out = new int[(-n) * 4];
            SCRATCH.set(out);
            // retry fused: re-upsert идемпотентен (тот же id/координаты)
            n = mobPushStep(boxId[0], lid, cx, cy, cz, hw, hh,
                    box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ, out);
            if (n >= 0) {
                fusedProofOnce();
                return collect(level, entity, box, out, n);
            }
            if (n == ERR_STRUCT) {
                broken = true;
                return vanillaFill(level, entity, box);
            }
            if (n <= OVERFLOW_MAX && gridProbeOnce()) {
                // self опубликован (первый upsert прошёл с rc=0) — grid
                // fallback равен legacy post-query лестнице
                int n2 = mobGridQuery(box.minX, box.minY, box.minZ,
                        box.maxX, box.maxY, box.maxZ, lid, out);
                if (n2 < 0 && n2 <= OVERFLOW_MAX) {
                    out = new int[(-n2) * 4];
                    SCRATCH.set(out);
                    n2 = mobGridQuery(box.minX, box.minY, box.minZ,
                            box.maxX, box.maxY, box.maxZ, lid, out);
                }
                if (n2 >= 0) {
                    return collect(level, entity, box, out, n2);
                }
            }
        }
        // ERR_RANGE: давление окна/таблицы индекса. upsert-pressure = self НЕ
        // опубликован в этом вызове — легаси-лестница upsert-rc!=0 = ваниль
        // на вызов (без grid-read); исчерпанные лестницы — туда же.
        return vanillaFill(level, entity, box);
    }

    /** One-shot effect-proof: отличает leg2/3-раны от leg1 в server-stdout. */
    private static void fusedProofOnce() {
        if (!fusedProof) {
            fusedProof = true;
            LOG.warning("[crussty-plugin] cmp403_jnibulk: mob-step fused ARMED "
                    + "(self-upsert+scan = 1 JNI transition/call, single-pass scan under writer lock)");
        }
    }

    /**
     * Замена сайта Level.getPushableEntities в LivingEntity.pushEntities
     * (ванильный хвост тела — cramming/numCollisions/doPush — работает по
     * возвращённому списку без изменений).
     */
    public static List<Entity> pushables(Level level, Entity entity, AABB box) {
        Profiler.get().incrementCounter("getEntities");
        // TASK-404-B: fused steady-state path (STRICT cmp403_jnibulk) —
        // self-upsert + candidate scan = ONE JNI transition instead of two.
        // Первый вызов сущности (idMap miss) проваливается в legacy-путь,
        // который аллоцирует/публикует id ровно как раньше.
        if (FUSED && !broken && !oversized) {
            List<Entity> fused = pushablesFused(level, entity, box);
            if (fused != null) {
                return fused;
            }
        }
        if (upsertSelf(entity) || broken) {
            return vanillaFill(level, entity, box);
        }
        int lid = System.identityHashCode(level);
        int[] out = SCRATCH.get();
        int n = mobQuery(box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ, lid, out);
        if (n < 0) {
            if (n <= OVERFLOW_MAX) {
                out = new int[(-n) * 4];
                SCRATCH.set(out);
                n = mobQuery(box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ, lid, out);
            }
            if (n < 0) {
                if (n == ERR_STRUCT) {
                    broken = true; // структурный отказ — весь рычаг дизармится
                    return vanillaFill(level, entity, box);
                }
                // ERR_RANGE / overflow-retry-fail: под композитом — retry
                // через зеркальный sharded grid (те же плотные id, тот же
                // byId, те же точные ванильные фильтры) ДО ванильного fill;
                // сама плоскость SoA не дизармится (per-call деградация).
                if (gridProbeOnce()) {
                    int n2 = mobGridQuery(box.minX, box.minY, box.minZ,
                            box.maxX, box.maxY, box.maxZ, lid, out);
                    if (n2 < 0 && n2 <= OVERFLOW_MAX) {
                        out = new int[(-n2) * 4];
                        SCRATCH.set(out);
                        n2 = mobGridQuery(box.minX, box.minY, box.minZ,
                                box.maxX, box.maxY, box.maxZ, lid, out);
                    }
                    if (n2 >= 0) {
                        return collect(level, entity, box, out, n2);
                    }
                }
                return vanillaFill(level, entity, box); // ERR_RANGE/overflow-retry-fail — per-call vanilla
            }
        }
        return collect(level, entity, box, out, n);
    }

    /**
     * Точная ванильная фильтрация кандидатов (level/AABB/pushableBy/other !=
     * entity) — общий хвост SoA-пути и зеркального grid-пути композита.
     */
    private static List<Entity> collect(Level level, Entity entity, AABB box, int[] out, int n) {
        ArrayList<Entity>[] ring = RING.get();
        int[] cursor = RING_CURSOR.get();
        int slot = cursor[0];
        cursor[0] = (slot + 1) % RING_SLOTS;
        ArrayList<Entity> list = ring[slot];
        list.clear();
        Predicate<Entity> predicate = EntitySelector.pushableBy(entity);
        for (int i = 0; i < n; i++) {
            int cid = out[i];
            if (cid < 0 || cid >= byId.length) {
                continue; // grow-гонка idMap/byId — bounds-guard
            }
            Entity other = byId[cid];
            if (other == null || other == entity || other.level() != level) {
                continue;
            }
            if (!other.getBoundingBox().intersects(box)) {
                continue; // точный ванильный box-тест
            }
            if (!predicate.test(other)) {
                continue; // ванильный предикат (pushableBy: isPushable/canCollideWith/team)
            }
            list.add(other);
        }
        maybeSweep();
        return list;
    }

    /**
     * Само-апсерт self в грид (id лениво). @return true — уйти в ваниль
     * (oversized / broken / структурный отказ).
     */
    private static boolean upsertSelf(Entity e) {
        if (oversized) {
            return true;
        }
        AABB bb = e.getBoundingBox();
        double hw = Math.max((bb.maxX - bb.minX) * 0.5D, (bb.maxZ - bb.minZ) * 0.5D);
        double hh = (bb.maxY - bb.minY) * 0.5D;
        if (Math.max(hw, hh) > 1.0D) {
            oversized = true; // не-грид-юниверс: весь рычаг в ваниль (fail-closed)
            LOG.warning("[crussty-plugin] cmp401_soa: oversized bounding radius "
                    + Math.max(hw, hh) + " on " + e.getType() + " — lever reverted to vanilla");
            return true;
        }
        double cx = (bb.minX + bb.maxX) * 0.5D;
        double cy = (bb.minY + bb.maxY) * 0.5D;
        double cz = (bb.minZ + bb.maxZ) * 0.5D;
        int lid = System.identityHashCode(e.level());
        int[] box = idMap.get(e);
        if (box != null) {
            int rc = mobUpsert(box[0], lid, cx, cy, cz, hw, hh);
            if (rc != 0) {
                if (rc == ERR_STRUCT) {
                    broken = true;
                }
                return true; // ненулевой rc — ваниль на этот вызов
            }
            return false;
        }
        synchronized (ID_LOCK) {
            box = idMap.get(e);
            if (box == null) {
                if (broken) {
                    return true;
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
                int rc = mobUpsert(id, lid, cx, cy, cz, hw, hh);
                if (rc != 0) {
                    if (rc == ERR_STRUCT) {
                        broken = true; // id не опубликован — ванильный путь
                    }
                    return true;   // ненулевой rc — id НЕ публикуем, ваниль на этот вызов
                }
                box = new int[] {id};
                byId[id] = e;
                idMap.put(e, box);
                return false;
            }
        }
        int rc = mobUpsert(box[0], lid, cx, cy, cz, hw, hh);
        if (rc != 0) {
            if (rc == ERR_STRUCT) {
                broken = true;
            }
            return true; // ненулевой rc — self не опубликован: ваниль на этот вызов
        }
        return false;
    }

    /**
     * Точная реплика fill-последовательности Level.getEntities(Entity,AABB,Predicate)
     * (javap: Profiler-счётчик уже инкрементирован выше; deep-fill
     * EntityLookup.getEntities + PlatformHooks.addToGetEntities; свежий
     * ArrayList вместо мёртвого guava-списка — EntityQueryOps-контракт).
     */
    private static List<Entity> vanillaFill(Level level, Entity entity, AABB box) {
        Predicate<Entity> predicate = EntitySelector.pushableBy(entity);
        ArrayList<Entity> list = new ArrayList<>();
        EntityLookup lookup = ((ChunkSystemLevel) (Object) level).moonrise$getEntityLookup();
        lookup.getEntities(entity, box, list, predicate);
        PlatformHooks.get().addToGetEntities(level, entity, box, predicate, list);
        return list;
    }

    // ------------------------------------------------------------------
    // Graveyard sweep: isRemoved()-энтити -> mobRemove + freeId (каденция
    // ~1/1024 вызова; атомарный single-sweeper; id reuse только под ID_LOCK).
    // ------------------------------------------------------------------
    private static final AtomicBoolean SWEEPING = new AtomicBoolean();
    private static long sweepGate = 0;

    private static void maybeSweep() {
        if (broken || (++sweepGate & 0x3FF) != 0L) {
            return; // sweepGate — каденция (racy инкремент с воркеров допустим)
        }
        if (!SWEEPING.compareAndSet(false, true)) {
            return;
        }
        try {
            int top = idTop;
            for (int id = 0; id < top; id++) {
                Entity e = byId[id];
                if (e == null || !e.isRemoved()) {
                    continue;
                }
                synchronized (ID_LOCK) {
                    Entity e2 = byId[id];
                    if (e2 == null || !e2.isRemoved()) {
                        continue;
                    }
                    byId[id] = null;
                    idMap.remove(e2);
                    int rc = mobRemove(id);
                    if (rc == ERR_STRUCT) {
                        broken = true;
                        return;
                    }
                    if (freeTop == freeIds.length) {
                        freeIds = java.util.Arrays.copyOf(freeIds, Math.max(16, freeTop * 2));
                    }
                    freeIds[freeTop++] = id;
                }
            }
        } catch (Throwable t) {
            LOG.warning("[crussty-plugin] cmp401_soa: sweep failed: " + t);
        } finally {
            SWEEPING.set(false);
        }
    }
}
