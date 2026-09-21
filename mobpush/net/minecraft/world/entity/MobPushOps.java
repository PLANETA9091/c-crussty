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
 * MOB-PUSH (TASK-400-J, vector mobpush — lever cmp399_mobpush).
 *
 * Rust-грид broadphase для мобового push-лейна. Порт семантики апстримного
 * Paper «optimize entity pushes»
 * (paper-server/patches/sources/net/minecraft/world/entity/LivingEntity.java.patch:
 * «don't run getEntities if we're not going to use its result» + «Cap entity
 * collisions»): ванильное тело LivingEntity.pushEntities потребляет результат
 * Level.getPushableEntities(this, this.getBoundingBox()) ТОЛЬКО циклом
 * doPush(entity) + cramming-счётчиками. Этот бридж заменяет ТОЛЬКО
 * перечисление кандидатов:
 *
 *   - позиции мобов пушатся в rust per-tick (mobUpsert: id + lid + центр AABB;
 *     bounding-радиус-гейт r_eff = max(halfW, halfH) <= 1.0 java-стороной —
 *     в бенч-популяции все мобы <= 0.7/2.0), без объектов — только плотный
 *     id->Entity массив java-стороны;
 *   - push-кандидаты отдаются java ОДНИМ запросом mobQuery(box, lid, out):
 *     шардированный seqlock-грид 1.0-ячеек (64 шарда mix64&63, v1->данные->v2
 *     Acquire, писатель под глобальным mutex, фикс. 16384 слота/шард, fail-
 *     closed на переполнении — pattern-B src/mobs_grid.rs);
 *   - точные ванильные фильтры на каждом кандидате: other.level() == level,
 *     AABB.intersects(box), EntitySelector.pushableBy(entity) (предикат
 *     ванильный — Scoreboard-лейн сохраняется), other != entity;
 *   - хвост LivingEntity.pushEntities (cramming + numCollisions + doPush) —
 *     НЕ ТРОНУТ ванильный байткод; смещения одного пуша идентичны ванильным
 *     по построению, порядок кандидатов = порядок цепочек ячеек (документир.
 *     дельта класса items_subsys2).
 *
 * СВЕЖЕСТЬ (без epsilon): pushEntities вызывается в aiStep ПОСЛЕ travel/move
 * (javap offsets 620..850), поэтому само-апсерт в момент запроса кладёт в грид
 * end-of-move позицию self; кандидаты, тикающие позже в этом же тике, ещё не
 * двигались (их грид-позиция = их прошлый end-of-tick = текущая). Взаимный
 * push(Entity) меняет только deltaMovement — позиция интегрируется собственным
 * move() пушуемого (Paper Entity.java.patch push(Entity): additive deltas) ⇒
 * living<->living свежесть точная в пределах однопроходного тика.
 *
 * FAIL-CLOSED: ENABLED (env == "cmp399_mobpush") && nativeOk (mobProbe magic)
 * && !broken (структурный отказ грида — дизарм навсегда) && !oversized (в
 * популяции замечен r_eff > 1.0 — весь рычаг в ванильный режим) — иначе
 * 100% ванильный fill (точная реплика fill-последовательности
 * Level.getEntities: Profiler-счётчик + EntityLookup.getEntities +
 * PlatformHooks.addToGetEntities, EntityQueryOps-контракт).
 * Пустой/чужой CRUSSTY_LEVER_FLAG — сайт вообще не ретаргетится (rust-сторона
 * не ставит патч), путь ванильный по построению.
 */
public final class MobPushOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp399_mobpush");
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x4D50; // "MP"

    /** Result codes natives: >=0 ok/count; -1 ERR_STRUCT (дизарм); -2 ERR_RANGE (per-call vanilla); -(cap) overflow. */
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    /** Overflow-порог: -(cap) при стартовом scratch 256 -> n <= -256; -2 не пересекается. */
    private static final int OVERFLOW_MAX = -256;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/mobs_grid.rs, RegisterNatives после define) ----
    private static native int mobProbe();
    private static native int mobUpsert(int id, int lid, double x, double y, double z);
    private static native int mobRemove(int id);
    private static native int mobQuery(double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int lid, int[] out);

    private static volatile boolean nativeOk;
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

    /** Gate для rust-стороны/диагностики: армирован ли бридж. */
    public static boolean armed() {
        return ENABLED && !broken && !oversized && probeOnce();
    }

    /**
     * Замена сайта Level.getPushableEntities в LivingEntity.pushEntities
     * (ванильный хвост тела — cramming/numCollisions/doPush — работает по
     * возвращённому списку без изменений).
     */
    public static List<Entity> pushables(Level level, Entity entity, AABB box) {
        Profiler.get().incrementCounter("getEntities");
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
                }
                return vanillaFill(level, entity, box); // ERR_RANGE/overflow-retry-fail — per-call vanilla
            }
        }
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
            LOG.warning("[crussty-plugin] cmp399_mobpush: oversized bounding radius "
                    + Math.max(hw, hh) + " on " + e.getType() + " — lever reverted to vanilla");
            return true;
        }
        double cx = (bb.minX + bb.maxX) * 0.5D;
        double cy = (bb.minY + bb.maxY) * 0.5D;
        double cz = (bb.minZ + bb.maxZ) * 0.5D;
        int lid = System.identityHashCode(e.level());
        int[] box = idMap.get(e);
        if (box != null) {
            int rc = mobUpsert(box[0], lid, cx, cy, cz);
            if (rc == ERR_STRUCT) {
                broken = true;
                return true;
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
                int rc = mobUpsert(id, lid, cx, cy, cz);
                if (rc == ERR_STRUCT) {
                    broken = true; // id не опубликован — ванильный путь
                    return true;
                }
                box = new int[] {id};
                byId[id] = e;
                idMap.put(e, box);
                return false;
            }
        }
        int rc = mobUpsert(box[0], lid, cx, cy, cz);
        if (rc == ERR_STRUCT) {
            broken = true;
            return true;
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
            LOG.warning("[crussty-plugin] cmp399_mobpush: sweep failed: " + t);
        } finally {
            SWEEPING.set(false);
        }
    }
}
