package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.damagesource.DamageSource;
import net.minecraft.world.level.GameRules;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.scores.PlayerTeam;
import net.minecraft.world.scores.Team;

/**
 * COLPUSH (TASK-419-A, round-419 vector A — lever cmp420_colpush).
 *
 * COLLIDE+PUSH ПОДСИСТЕМА ЦЕЛИКОМ Java→Rust: заменяет ПЕР-СУЩНОСТНУЮ лестницу
 * push-лейна master (MobPushOps.pushables → upsertSelf JNI + eqsnap chain
 * скан + collect-аллокации на КАЖДОГО living entity В КАЖДЫЙ aiStep,
 * ~40-48k JNI-переходов/тик) на ОДИН bulk-JNI/тик:
 *
 *  1. ВХОДНОЙ БУФЕР: персистентные java-массивы COL_D double[idTop*6]
 *     (cx,cy,cz,hx,hz,hh — ТОЧНЫЕ осевые полуэкстенты) + COL_I int[idTop*3]
 *     (lid, flags, freshTick). Пишутся per-entity прямо в pushEntities
 *     (простые store: 0 JNI, 0 аллокаций), id = плотный id mobs_soa
 *     (MobPushOps.boxFor — ТОТ ЖЕ юниверс, что byArr()/idCount() у
 *     eqsnap/sscan/ai-плоскостей). Flags bit0 (FLAG_INCLUDE) = covered:
 *     isPushable && alive && !spectator && !noPhysics && team==null &&
 *     !vehicle && !passenger && bounding-radius <= 2.0 (k4soa-гейт).
 *  2. ОДИН bulk-JNI/тик colpushTick(tick, idTop, COL_D, COL_I, OFF, IDS)
 *     — main-поток, RegionTickOps.forEach ДО GO-барьера (0 гонок/конвоя):
 *     rust строит активный список (fresh == tick-1 = end-of-previous-tick
 *     снапшот позиций — никто ещё не тикал), uniform-grid broad-phase
 *     (ячейка 4.0, pad 1, reach <= hx_i+hx_j <= 4.0), точный AABB-overlap
 *     (строгие <) и пишет CSR списки кандидатов (по возрастанию плотного id)
 *     в OFF/IDS. Заодно ОДИН WLOCK-рефреш колонок mobs_soa
 *     (colpush_plane_refresh: sscan/ai/eq read-views живы, 0 per-entity JNI).
 *  3. ВЫХОДНОЙ ХВОСТ: ваниль бит-в-байт — живая ревалидация кандидатов
 *     (level/!=self/AABB.intersects/pushableBy — контракт MobPushOps.collect),
 *     cramming (RNG self.random.nextInt(4), nonPass, hurtServer 6.0f),
 *     numCollisions (живые поля: кредит max(0, nc-maxCol), cross-entity
 *     cap-выход, ++ обоим), e.push(self) (= ванильный doPush: живые позиции,
 *     живое округление per-pair add, hasImpulse).
 *
 * ПАРНОСТЬ (закон 4): пустой/чужой CRUSSTY_LEVER_FLAG — класс вообще не
 * определяется rust-стороной, хук спит, LivingEntity бит-в-байт ванильный.
 * FAIL-CLOSED: broken (probe/colpushTick не пройдёт / Throwable) — каждая
 * ветка уводится в ТОЧНУЮ ваниль-реплику тела pushEntities (fetch через
 * Level.getPushableEntities). oversized (r_eff > 2.0 — хроника round-406d..410)
 * — весь рычаг в ваниль навсегда (контракт MobPushOps.upsertSelf).
 *
 * DELTA (задокументировано, класс ghost-контракта eqsnap master):
 *  - кандидаты из end-of-previous-tick позиций (<=1 тик ghost);
 *  - порядок кандидатов = возрастание плотного id (вместо live chain-порядка);
 *  - passengers/vehicles/teamed/oversized исключены из плоскости (fail-closed
 *    per-entity ваниль для себя; в bench-популяции ~0);
 *  - cramming-count по снапшоту (граничные +-1 у порога RNG-ветки);
 *  - graveyard-sweep каденция 1/1024 bulkTick (master: 1/1024 pushables
 *    вызова ~ 47/тик) — isRemoved-строки живут в плоскости до ~51 c.
 */
public final class ColpushOps {

    private ColpushOps() {}

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    private static final String FLAG = "cmp420_colpush";
    /** TASK-426-A: SoA-feed carrier — STRICT-OR (будит java-сторону колпаша
     * под вектор-флагом; без него ENABLED=false = спящий гейт, урок ×93). */
    private static final String FLAG2 = "cmp424_mobfeed";
    /** TASK-430-B: inside-plane subsystem round rides the carrier (STRICT-OR;
     * constant-pool marker for the check_blobs_sync raw-byte gate, x93). */
    private static final String FLAG3 = "cmp430_inside";
    /** TASK-434-C: chunk-pipeline R5 carrier (STRICT-OR; raw-cp marker
     * for the check_blobs_sync gate, x93). */
    private static final String FLAG4 = "cmp434_chunkpl";
    /** TASK-435-C: R6 carrier (STRICT-OR; raw-cp marker for the
     * check_blobs_sync gate). */
    private static final String FLAG5 = "cmp435_chunk3";

    /**
     * TASK-438-C chunk-pipeline R7 carrier (law 7/8): STRICT-OR successor id
     * ON TOP of cmp435_chunk3 — the composite now also carries the chunk-send
     * serialization snapshot plane (round-id hygiene for ROUND-438-C).
     */
    private static final String FLAG6 = "cmp437_chunk4";
    /** TASK-444-B: R8 stage-2 carrier (STRICT-OR; raw-cp marker for the
     * check_blobs_sync gate). */
    private static final String FLAG7 = "cmp444_chunk5";
    /** TASK-450-C union carrier (chunk4⊕chunk5⊕slices). */
    private static final String FLAG8 = "cmp450_chunk";
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    /** Row layout — src/colpush.rs ROW_D/ROW_I/FLAG_INCLUDE contract. */
    static final int ROW_D = 6, ROW_I = 3;
    static final int FLAG_INCLUDE = 1;
    /** Плотный-id cap (тот же, что rust IDS_CAP / mobs_soa IDS_CAP). */
    static final int IDS_CAP = 1 << 20;
    /** k4soa-гейт населения: max(hx,hz,hh) <= 2.0 (пара с rust CELL=4.0/pad 1). */
    static final double RADIUS_GATE = 2.0D;

    private static final boolean ENABLED = leverEnabled();

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && (f.trim().equals(FLAG) || f.trim().equals(FLAG2) || f.trim().equals(FLAG3) || f.trim().equals(FLAG4) || f.trim().equals(FLAG5) || f.trim().equals(FLAG6) || f.trim().equals(FLAG7) || f.trim().equals(FLAG8));
    }

    /** Структурный отказ — весь рычаг дизармится навсегда (ваниль-реплика). */
    private static volatile boolean broken = false;
    /** Oversized-дизарм (весь рычаг, контракт MobPushOps.upsertSelf). */
    private static volatile boolean oversized = false;
    private static volatile boolean nativeOk = false;

    // ---- natives (impl: src/colpush.rs, RegisterNatives после define) ----
    private static native int colpushProbe();
    private static native int colpushTick(int tick, int idTop, double[] inD,
            int[] inI, int[] outOff, int[] outIds);

    // ---- персистентные буферы (лениво в bulkTick, main-поток, ДО GO-барьера:
    // воркеры ещё не тикают — 0 гонок роста) ----
    private static double[] colD;
    private static int[] colI;
    private static int[] off;
    private static int[] ids;

    /** Кольцо ревалидации (EntityQueryOps/MobPushOps-паттерн: 4 слота). */
    private static final int RING_SLOTS = 4;
    private static final ThreadLocal<ArrayList<Entity>[]> RING =
            ThreadLocal.withInitial(ColpushOps::makeRing);
    private static final ThreadLocal<int[]> RING_CURSOR =
            ThreadLocal.withInitial(() -> new int[1]);

    @SuppressWarnings("unchecked")
    private static ArrayList<Entity>[] makeRing() {
        ArrayList<Entity>[] ring = new ArrayList[RING_SLOTS];
        for (int i = 0; i < RING_SLOTS; i++) {
            ring[i] = new ArrayList<>();
        }
        return ring;
    }

    private static ArrayList<Entity> ring() {
        ArrayList<Entity>[] ring = RING.get();
        int[] cursor = RING_CURSOR.get();
        int slot = cursor[0];
        cursor[0] = (slot + 1) % RING_SLOTS;
        ArrayList<Entity> list = ring[slot];
        list.clear();
        return list;
    }

    /** Одноразовый EFFECT-маркер push-хвоста (server-stdout grep). */
    private static volatile boolean EFFECT_LOGGED = false;

    private static boolean nativeProbeOnce() {
        if (nativeOk) {
            return true;
        }
        if (broken) {
            return false;
        }
        synchronized (ColpushOps.class) {
            if (nativeOk || broken) {
                return nativeOk;
            }
            try {
                nativeOk = colpushProbe() == 0x435050;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    // ------------------------------------------------------------------
    // Redirect target: LivingEntity.pushEntities (whole-body, static form
    // receiver-prepended — контракт redirect_method_body_to_static).
    // ------------------------------------------------------------------
    public static void pushEntities(LivingEntity self) {
        if (broken) {
            vanillaReplica(self);
            return;
        }
        try {
            pushEntitiesImpl(self);
        } catch (Throwable t) {
            broken = true;
            LOG.warning("[crussty-plugin] " + FLAG + ": pushEntities threw " + t + " — disarmed to vanilla");
            vanillaReplica(self);
        }
    }

    private static void pushEntitiesImpl(LivingEntity self) {
        // --- ванильные гейты (bit-в-байт, offsets 0..65 дизасма) ---
        if (!self.isPushable()) {
            return;
        }
        Level level = self.level();
        PlayerTeam team = self.getTeam();
        if (team != null && team.getCollisionRule() == Team.CollisionRule.NEVER) {
            return;
        }
        int cramming = ((ServerLevel) level).getGameRules().getInt(GameRules.RULE_MAX_ENTITY_CRAMMING);
        int maxCol = level.paperConfig().collisions.maxEntityCollisions;
        if (cramming <= 0 && maxCol <= 0) {
            return;
        }

        // --- входной буфер: строка self (0 JNI, простые store) ---
        int id = -1;
        double[] d = colD;
        int[] ii = colI;
        if (d != null && !oversized) {
            AABB bb = self.getBoundingBox();
            double hx = (bb.maxX - bb.minX) * 0.5D;
            double hz = (bb.maxZ - bb.minZ) * 0.5D;
            double hh = (bb.maxY - bb.minY) * 0.5D;
            if (Math.max(Math.max(hx, hz), hh) > RADIUS_GATE) {
                oversized = true;
                LOG.warning("[crussty-plugin] " + FLAG + ": oversized bounding radius "
                        + Math.max(Math.max(hx, hz), hh) + " on " + self.getType()
                        + " — lever reverted to vanilla");
            } else {
                int[] box = MobPushOps.boxFor(self);
                if (box != null && box[0] >= 0 && box[0] < IDS_CAP) {
                    id = box[0];
                    int b = id * ROW_D;
                    d[b] = (bb.minX + bb.maxX) * 0.5D;
                    d[b + 1] = (bb.minY + bb.maxY) * 0.5D;
                    d[b + 2] = (bb.minZ + bb.maxZ) * 0.5D;
                    d[b + 3] = hx;
                    d[b + 4] = hz;
                    d[b + 5] = hh;
                    int ib = id * ROW_I;
                    ii[ib] = System.identityHashCode(level);
                    ii[ib + 1] = FLAG_INCLUDE;
                    ii[ib + 2] = (int) net.minecraft.server.MinecraftServer.getServer().getTickCount();
                }
            }
        }

        if (id < 0) {
            // non-covered self (буферы холодные / oversized / id-cap) — точная
            // ваниль-реплика: fetch через Level.getPushableEntities.
            vanillaReplica(self);
            return;
        }

        // --- bulk-CSR кандидаты (написаны rust'ом ДО GO-барьера этого тика) ---
        Profiler.get().incrementCounter("getEntities");
        int n = off[id + 1] - off[id];
        if (n == 0) {
            return; // ваниль: пустой список = return ДО RNG/декремента
        }

        // Живая ревалидация: ТОЧНЫЕ ванильные фильтры MobPushOps.collect
        // (level/!=self/AABB.intersects/pushableBy) по ЖИВЫМ боксам.
        ArrayList<Entity> list = ring();
        AABB sbb = self.getBoundingBox();
        Predicate<Entity> pred = EntitySelector.pushableBy(self);
        Entity[] byId = MobPushOps.byIdArr();
        int base = off[id];
        for (int k = 0; k < n; k++) {
            int cid = ids[base + k];
            if (cid < 0 || cid >= byId.length) {
                continue; // grow-гонка idMap/byId — bounds-guard
            }
            Entity other = byId[cid];
            if (other == null || other == self || other.level() != level) {
                continue;
            }
            if (!other.getBoundingBox().intersects(sbb)) {
                continue; // точный ванильный box-тест
            }
            if (!pred.test(other)) {
                continue; // ванильный предикат (pushableBy: isPushable/canCollideWith/team)
            }
            list.add(other);
        }
        if (list.isEmpty()) {
            return; // все кандидаты отвалились — эквивалент ванильного isEmpty
        }
        if (!EFFECT_LOGGED) {
            EFFECT_LOGGED = true;
            LOG.info("[crussty-plugin] " + FLAG + ": push-plane EFFECT armed (first gate hit tick "
                    + net.minecraft.server.MinecraftServer.getServer().getTickCount()
                    + ", candidates=" + n + ", live=" + list.size() + ", zero per-entity JNI)");
        }

        // --- ванильный хвост bit-в-байт (offsets 88..319 дизасма) ---
        crammingAndPushTail(self, level, list, cramming, maxCol);
    }

    /** Точная ваниль-реплика тела pushEntities (fail-closed ветка). */
    private static void vanillaReplica(LivingEntity self) {
        if (!self.isPushable()) {
            return;
        }
        Level level = self.level();
        PlayerTeam team = self.getTeam();
        if (team != null && team.getCollisionRule() == Team.CollisionRule.NEVER) {
            return;
        }
        int cramming = ((ServerLevel) level).getGameRules().getInt(GameRules.RULE_MAX_ENTITY_CRAMMING);
        int maxCol = level.paperConfig().collisions.maxEntityCollisions;
        if (cramming <= 0 && maxCol <= 0) {
            return;
        }
        List<Entity> list = level.getPushableEntities(self, self.getBoundingBox());
        if (list.isEmpty()) {
            return;
        }
        crammingAndPushTail(self, level, list, cramming, maxCol);
    }

    /**
     * Ванильный хвост (cramming + numCollisions + doPush) — общий для CSR- и
     * реплики-веток; sequence = дизасм offsets 88..319 bit-в-байт.
     */
    private static void crammingAndPushTail(LivingEntity self, Level level, List<Entity> list,
            int cramming, int maxCol) {
        if (level instanceof ServerLevel sl && cramming > 0 && list.size() > cramming - 1
                && self.random.nextInt(4) == 0) {
            int nonPass = 0;
            for (int k = 0; k < list.size(); k++) {
                if (!list.get(k).isPassenger()) {
                    nonPass++;
                }
            }
            if (nonPass > cramming - 1) {
                DamageSource src = self.damageSources().cramming();
                self.hurtServer(sl, src, 6.0F);
            }
        }
        self.numCollisions = Math.max(0, self.numCollisions - maxCol);
        for (int k = 0; k < list.size(); k++) {
            if (self.numCollisions >= maxCol) {
                break; // ванильный cap-выход
            }
            Entity e = list.get(k);
            e.numCollisions++;
            self.numCollisions++;
            e.push(self); // ванильный doPush: живые позиции/округление/hasImpulse
        }
    }

    // ------------------------------------------------------------------
    // ОДИН bulk-JNI/тик: триггерится RegionTickOps.forEach (main-поток, ДО
    // GO-барьера фазы воркеров). Rust: fresh == tick-1 (end-of-previous-tick
    // снапшот), grid broad-phase, точный overlap, CSR в OFF/IDS + ОДИН
    // WLOCK-рефреш колонок mobs_soa (sscan/ai/eq read-views).
    // ------------------------------------------------------------------
    public static void bulkTick() {
        if (broken || !ENABLED) {
            return;
        }
        // TASK-420-A (FIX-МАНДАТ п.2, belt-and-braces): ОДНОРАЗОВЫЙ guard
        // вокруг ВСЕГО тела — первый Throwable (в т.ч. NCDFE любой поздней
        // резолюции) = eprintln ×1 + vanilla-хвост (pushEntities fail-closed
        // уже ванильная реплика) + DISARM lever навсегда (RegionTickOps
        // COLPUSH_ON=false ⇒ call site мёртв): шторм ×1902 невозможен.
        try {
            bulkTickImpl();
        } catch (Throwable t) {
            broken = true;
            RegionTickOps.COLPUSH_ON = false;
            RegionTickOps.COLPUSH_BROKEN = true;
            LOG.warning("[crussty-plugin] " + FLAG + ": bulkTick one-shot guard — " + t
                    + " — DISARM (vanilla tail, no storm)");
        }
    }

    private static void bulkTickImpl() {
        if (colD == null) {
            colD = new double[IDS_CAP * ROW_D];
            colI = new int[IDS_CAP * ROW_I];
            off = new int[IDS_CAP + 1];
            ids = new int[1 << 21];
            LOG.info("[crussty-plugin] " + FLAG + ": buffers allocated (colD "
                    + (colD.length >> 10) + "K doubles, ids " + (ids.length >> 20)
                    + "M ints, selfTest=" + selfTest() + ")");
        }
        if (!nativeProbeOnce()) {
            broken = true;
            LOG.warning("[crussty-plugin] " + FLAG + ": probe failed — disarmed to vanilla");
            return;
        }
        int idTop = MobPushOps.idCount();
        if (idTop <= 0) {
            return; // холодная плоскость
        }
        if (idTop > IDS_CAP) {
            idTop = IDS_CAP; // сверх-cap ряды не обслуживаются (mobs_soa-контракт)
        }
        int tick = (int) net.minecraft.server.MinecraftServer.getServer().getTickCount();
        int rc;
        try {
            rc = colpushTick(tick, idTop, colD, colI, off, ids);
        } catch (Throwable t) {
            broken = true;
            LOG.warning("[crussty-plugin] " + FLAG + ": colpushTick threw " + t + " — disarmed");
            return;
        }
        if (rc == ERR_STRUCT) {
            broken = true;
            LOG.warning("[crussty-plugin] " + FLAG + ": colpushTick ERR_STRUCT — disarmed to vanilla");
        } else if (rc == ERR_RANGE) {
            // pair-overflow: растим ids (одна сторона) — ретрай СЛЕДУЮЩЕГО
            // тика (проход идемпотентен, буферы персистентны).
            if (ids.length < (1 << 24)) {
                ids = java.util.Arrays.copyOf(ids, ids.length * 2);
            }
        }
        // Graveyard sweep (isRemoved -> mobRemove + freeId): каденция 1/1024
        // вызова внутри MobPushOps.maybeSweep (bulkTick = 1/тик => 1/1024 тика).
        MobPushOps.colpushSweep();
    }

    /**
     * Оффлайн-инварианты буферов/юниверса (без JNI): row-layout, cap-контракт
     * с rust (IDS_CAP/ROW_D/ROW_I), доступность общего плотного юниверса
     * MobPushOps. Вызывается один раз из bulkTick (маркер в stdout).
     */
    public static boolean selfTest() {
        // TASK-427 ROOT-CAUSE FIX: прежняя версия содержала
        // `IDS_CAP * ROW_D < (1 << 31)` — в int-арифметике 1<<31 =
        // Integer.MIN_VALUE, поэтому сравнение 6291456 < -2147483648 ЛОЖНО
        // ещё на этапе КОМПИЛЯЦИИ; javac сворачивал всю &&-цепочку в
        // константу false (bytecode: iconst_0) => selfTest ВСЕГДА возвращал
        // false => colpush hook dormant fail-closed на каждом пересобранном
        // блобе (mobfeed/chunksend) => SoA-кормление не просыпалось
        // (mobSlots=0 x895, DATA-PLAN FAIL). Лечение: long-арифметика
        // (1L<<31 = 2147483648) — конъюнкты становятся константно-истинными,
        // рантайм-проверки MobPushOps остаются живыми в байткоде.
        boolean ok = ROW_D == 6 && ROW_I == 3 && FLAG_INCLUDE == 1
                && IDS_CAP == (1 << 20)
                && (long) IDS_CAP * (long) ROW_D < (1L << 31)
                && (long) IDS_CAP * (long) ROW_I < (1L << 31)
                && MobPushOps.idCount() >= 0 && MobPushOps.idCapacity() > 0
                && MobPushOps.byIdArr() != null;
        if (!ok) {
            LOG.warning("[crussty-plugin] " + FLAG + ": selfTest FAILED (buffers/universe mismatch)");
        }
        return ok;
    }

    /** Гейт для rust-стороны/диагностики: бридж жив и не сломан. */
    public static boolean armed() {
        return ENABLED && !broken && nativeOk;
    }
}
