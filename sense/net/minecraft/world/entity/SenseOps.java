package net.minecraft.world.entity;

import java.util.List;
import java.util.logging.Logger;

import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.ServerEntityGetter;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.ai.targeting.TargetingConditions;
import net.minecraft.world.entity.player.Player;

/**
 * SENSE-PLANE (TASK-438-A2 restart of TASK-438-A, vector cmp438_sense).
 *
 * TARGETING NEAREST-PICK CHOKEPOINT: javap ground truth (patched-kernel.jar
 * round-396-a, purpur-1.21.10) — ВСЯ targeting-conditions nearest-выборка
 * сходится в ОДИН интерфейсный default-метод
 * {@code ServerEntityGetter.getNearestEntity(List, TargetingConditions,
 * LivingEntity, double, double, double)}: четыре getNearestPlayer-варианта +
 * getNearestEntity(Class) + getNearestEntity(TagKey) депегаются в него,
 * ServerLevel НЕ переопределяет, NearestAttackableTargetGoal.findTarget зовёт
 * напрямую. Rust body-swap (classfile.rs patch_sense_nearest_entity) заменяет
 * тело 14-байтной straight line на статический мост
 * {@link #nearestEntityGate}. Ванильное тело (javap offsets 0..97):
 *
 * <pre>
 * double d = -1.0; LivingEntity best = null;
 * for (LivingEntity c : list) {
 *     if (!conditions.test(getter.getLevel(), targeter, c)) continue;
 *     double e = c.distanceToSqr(x, y, z);
 *     if (d == -1.0 || e &lt; d) { d = e; best = c; }
 * }
 * return best;
 * </pre>
 *
 * РЕШЕНИЕ (rust = единственный источник): ОДИН bulk-JNI
 * {@code senseEpoch(tick, idTop, players[D], nearest[I])} за тик — rust
 * DOD-проходом по SoA-позициям mobs_soa (sscan_snapshot reader-контракт,
 * seqlock=global-version, bounded retry) пишет колонку
 * {@code nearest[denseId] = индекс ближайшего игрока | -1} в разделяемый
 * java int[]. Гейт = ТОЧНАЯ ванильная лестница + два decision-exact
 * ускорения: (1) rust-guided best-first — rust-игрок R тестируется ПЕРВЫМ
 * (identity-guard: R обязан быть элементом списка кандидатов, O(n)), что
 * устанавливает bestD до основного свипа; (2) дистанция-обрезка свипа —
 * кандидаты с {@code d > bestD} НЕ МОГУТ выиграть строгую лестницу
 * {@code (e < d)} и пропускают тест; связки {@code d == bestD} ПОЗЖЕ
 * установленного best тоже пропускаются (ваниль хранит более ранний), а
 * связки РАНЬШЕ R — тестируются (ваниль может отдать их). TargetingConditions.test
 * чист (range/invisibility/idleTimeout/selector/LOS-clip; RandomSource НЕ
 * потребляется) ⇒ любой порядок тестов + пропуск непобеждающих кандидатов
 * дают ванильный pick: min-dist passing, tie-keep-earlier. STALE снапшот
 * деградирует ТОЛЬКО выигрыш (R не пройдёт тест → полный свип), не
 * корректность (закон 4). Решение проверяется exhaustive-оракулом
 * corePick-vs-naivePick в {@link #selfTest()} (см. также
 * sense/harness — локальный lockstep-прогон).
 *
 * FAIL-CLOSED: ENABLED STRICT-OR (пустой/чужой флаг — сайт вообще не
 * ретаргетится rust-стороной); не-Mob targeter / вне SoA-плоскости
 * (idBoxOf == null) / id вне последней эпохи → полный ванильный свип;
 * пустой список игроков на тике → колонка не публикуется, гейт идёт свипом;
 * senseProbe magic-mismatch / senseEpoch ERR_STRUCT → дизарм навсегда
 * (vanillaReplica = байт-точная ваниль-реплика тела); ERR_RANGE / эпоха-промах
 * → свип на этот тик (эпоха ретраится на следующем). Снапшот публикуется
 * volatile-четвёркой (SNAPSHOT, NEAREST, NEAREST_LEN) c EPOCH_TICK как
 * release-edge — читатели (гейт) видят консистентную тройку.
 */
public final class SenseOps {

    private static boolean leverEnabled() {
        // env-хатч (мандат cmp438_sense): прямой CRUSSTY_SENSE=1 без lever-флага.
        String hatch = System.getenv("CRUSSTY_SENSE");
        if (hatch != null
                && (hatch.trim().equalsIgnoreCase("1") || hatch.trim().equalsIgnoreCase("true")
                        || hatch.trim().equalsIgnoreCase("on") || hatch.trim().equalsIgnoreCase("yes"))) {
            return true;
        }
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // STRICT OR (TASK-402-F урок полу-armed гейта): только точный флаг
        // cmp438_sense или семейство-носители cmp430_inside-эры — ЗЕРКАЛО
        // src/mobs_sense.rs enabled() (расхождение = дормант-мисс ARM).
        return f != null && (f.trim().equals("cmp438_sense")
                || f.trim().equals("cmp406_sscan") || f.trim().equals("cmp409_multi")
                || f.trim().equals("cmp412_meganav") || f.trim().equals("cmp412_eqsnapv3")
                || f.trim().equals("cmp414_cvs") || f.trim().equals("cmp417_bq")
                || f.trim().equals("cmp420_colpush") || f.trim().equals("cmp421_brain")
                || f.trim().equals("cmp422_brain2") || f.trim().equals("cmp423_brain3")
                || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside")
                || f.trim().equals("cmp451_senseins") || f.trim().equals("cmp458_swar") || f.trim().equals("cmp463_swar_hilbert") || f.trim().equals("cmp457_paldelta") || f.trim().equals("cmp457_eqsnap2") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp453_diet") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_chunkmono") || f.trim().equals("cmp456_chunkmono_p31snap"));
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x5345; // "SE"
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    static final Logger LOG = Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/mobs_sense.rs, RegisterNatives после define) ----
    private static native int senseProbe();
    private static native int senseEpoch(int tick, int idTop, double[] players, int[] nearest);

    private static volatile boolean nativeOk;
    private static volatile boolean broken;

    /** Снапшот игроков (порядок getter.players() сохранён — ванильный порядок). */
    private static volatile Player[] SNAPSHOT = new Player[0];
    /** Колонка nearest[denseId] -> индекс в SNAPSHOT | -1, публикуется с NEAREST_LEN. */
    private static volatile int[] NEAREST = new int[1024];
    /** Сколько валидных элементов записал rust в последний senseEpoch. */
    private static volatile int NEAREST_LEN = 0;
    /** Пустой набор игроков на этот тик: колонка бесполезна, гейт идёт свипом. */
    private static volatile boolean SNAPSHOT_EMPTY = false;
    /** Серверный тик последней успешной эпохи (double-checked locking). */
    private static volatile long EPOCH_TICK = Long.MIN_VALUE;
    private static final Object EPOCH_LOCK = new Object();

    /** One-shot ARM/effect-пруфы (видны в server-stdout.log). */
    private static volatile boolean EFFECT_LOGGED = false;
    private static volatile boolean EPOCH_LOGGED = false;

    /** Метка активного флага для ARM/EFFECT-строк (TASK-437-A discipline). */
    private static final String LABEL = label();

    private static String label() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f == null ? "(off)" : f.trim();
    }

    private SenseOps() {}

    /** Ленивая проверка натива (Throwable -> false, ретрай на следующем тике). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (SenseOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = senseProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Диагностика rust-стороны: дизарм ли плейн. */
    public static boolean armed() {
        return ENABLED && !broken && probeOnce();
    }

    /**
     * Замена тела {@code ServerEntityGetter.getNearestEntity(List, TC, LE, DDD)}
     * (default-метод, receiver = getter). Stack-identical замещение: desc РОВНО
     * (Lnet/minecraft/server/level/ServerEntityGetter;Ljava/util/List;Lnet/
     * minecraft/world/entity/ai/targeting/TargetingConditions;Lnet/minecraft/
     * world/entity/LivingEntity;DDD)Lnet/minecraft/world/entity/LivingEntity; —
     * virtual desc с receiver-классом, препендированным (валидатор compose).
     */
    public static LivingEntity nearestEntityGate(ServerEntityGetter getter,
            List<? extends LivingEntity> entities, TargetingConditions conditions,
            LivingEntity targeter, double x, double y, double z) {
        if (ENABLED && !broken && getter != null && conditions != null && entities != null) {
            try {
                return nearestEntityGateImpl(getter, entities, conditions, targeter, x, y, z);
            } catch (Throwable th) {
                broken = true; // структурный отказ — весь рычаг в ваниль навсегда
                LOG.warning("[crussty-plugin] " + LABEL
                        + ": sense gate threw (" + th + ") — targeting disarmed to vanilla");
            }
        }
        return vanillaReplica(getter, entities, conditions, targeter, x, y, z);
    }

    /** Ускоренная лестница — rule-for-rule транскрипция {@link #corePick}. */
    private static LivingEntity nearestEntityGateImpl(ServerEntityGetter getter,
            List<? extends LivingEntity> entities, TargetingConditions conditions,
            LivingEntity targeter, double x, double y, double z) {
        maybeEpoch(getter);
        if (broken) {
            return vanillaReplica(getter, entities, conditions, targeter, x, y, z);
        }

        // ---- rust-guided establish: R = колонка ближайшего игрока ----
        LivingEntity r = null;
        int idxR = -1;
        if (!SNAPSHOT_EMPTY) {
            int id = -1;
            if (targeter instanceof Mob mob) {
                int[] box = MobPushOps.idBoxOf(mob);
                if (box != null) {
                    id = box[0];
                }
            }
            if (id >= 0) {
                int[] col = NEAREST;
                int len = NEAREST_LEN;
                if (col != null && id < len && id < col.length) {
                    int pidx = col[id];
                    if (pidx >= 0) {
                        Player[] ps = SNAPSHOT;
                        if (pidx < ps.length) {
                            r = ps[pidx];
                        }
                    }
                }
            }
            if (r != null) {
                // identity-guard: R обязан быть элементом списка кандидатов
                // (список может быть AABB-подмножеством или не игроками вовсе).
                int i = 0;
                for (LivingEntity c : entities) {
                    if (c == r) {
                        idxR = i;
                        break;
                    }
                    i++;
                }
            }
        }
        double bestD = -1.0;
        LivingEntity best = null;
        int idxBest = -1;
        boolean rFailed = false;
        if (idxR >= 0) {
            double dR = r.distanceToSqr(x, y, z);
            if (conditions.test(getter.getLevel(), targeter, r)) {
                bestD = dR;
                best = r;
                idxBest = idxR;
                if (!EFFECT_LOGGED) {
                    EFFECT_LOGGED = true;
                    LOG.info("[crussty-plugin] " + LABEL + ": sense EFFECT armed (first gate hit"
                            + " at tick " + MinecraftServer.getServer().getTickCount()
                            + ", denseId-candidates=" + entities.size() + ")");
                }
            } else {
                rFailed = true; // R уже оттестирован (чистый предикат) — повтор не нужен
            }
        }

        // ---- ускоренная ванильная лестница (полный свип, EXACT ties/prune) ----
        int i = 0;
        for (LivingEntity c : entities) {
            if (best != null) {
                if (i != idxBest) {
                    double di = c.distanceToSqr(x, y, z);
                    if (di <= bestD) {
                        boolean tie = di == bestD;
                        // !tie: строго ближе — может выиграть лестницу.
                        // tie && i < idxBest: связка РАНЬШЕ best — ваниль хранит её.
                        // (tie && i > idxBest отсечено условием di <= bestD + check.)
                        if (!tie || i < idxBest) {
                            if (conditions.test(getter.getLevel(), targeter, c)) {
                                if (!tie) {
                                    bestD = di;
                                }
                                best = c;
                                idxBest = i;
                            }
                        }
                    }
                }
            } else {
                boolean p = !(rFailed && i == idxR);
                if (p) {
                    p = conditions.test(getter.getLevel(), targeter, c);
                }
                if (p) {
                    bestD = c.distanceToSqr(x, y, z);
                    best = c;
                    idxBest = i;
                }
            }
            i++;
        }
        return best;
    }

    /**
     * Байт-точная ваниль-реплика тела getNearestEntity(List, TC, LE, DDD)
     * (javap offsets 0..97) — fail-closed путь (дизарм/throw).
     */
    private static LivingEntity vanillaReplica(ServerEntityGetter getter,
            List<? extends LivingEntity> entities, TargetingConditions conditions,
            LivingEntity targeter, double x, double y, double z) {
        double d = -1.0;
        LivingEntity best = null;
        for (LivingEntity c : entities) {
            if (!conditions.test(getter.getLevel(), targeter, c)) {
                continue;
            }
            double e = c.distanceToSqr(x, y, z);
            if (d == -1.0 || e < d) {
                d = e;
                best = c;
            }
        }
        return best;
    }

    /**
     * Одна эпоха сенса на серверный тик: снапшот игроков (порядок
     * getter.players() — ванильный; БЕЗ фильтра — TargetingConditions у каждого
     * вызова свои, гейт применяет тест к R и свипу сам) + ОДИН bulk-JNI по всей
     * SoA-популяции (rust пишет колонку nearest). Double-checked по volatile
     * EPOCH_TICK. rc >= 0 = число записанных валидных элементов.
     */
    private static void maybeEpoch(ServerEntityGetter getter) {
        long t = MinecraftServer.getServer().getTickCount();
        if (EPOCH_TICK == t) {
            return; // горячий путь: один volatile-read
        }
        synchronized (EPOCH_LOCK) {
            if (EPOCH_TICK == t || broken) {
                return;
            }
            List<? extends Player> ps;
            try {
                ps = getter.players();
            } catch (Throwable th) {
                return; // список кинул — этот тик свипом (эпоха не публикуется)
            }
            int np = ps == null ? 0 : ps.size();
            if (np == 0) {
                SNAPSHOT_EMPTY = true;
                EPOCH_TICK = t;
                return;
            }
            double[] coords = new double[np * 3];
            Player[] arr = ps.toArray(new Player[0]);
            for (int i = 0; i < arr.length; i++) {
                Player p = arr[i];
                coords[i * 3] = p.getX();
                coords[i * 3 + 1] = p.getY();
                coords[i * 3 + 2] = p.getZ();
            }
            int idTop = MobPushOps.idCount();
            if (idTop <= 0) {
                // SoA холодная — свип без JNI/буферов на этот тик (эпоха помечена).
                SNAPSHOT_EMPTY = true;
                EPOCH_TICK = t;
                return;
            }
            int[] col = NEAREST;
            int cap = Math.max(1024, MobPushOps.idCapacity());
            if (col == null || col.length < cap) {
                col = new int[cap];
            }
            int rc;
            try {
                rc = senseEpoch((int) t, idTop, coords, col);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                broken = true; // структурный отказ — весь рычаг в ваниль навсегда
                LOG.warning("[crussty-plugin] " + LABEL
                        + ": senseEpoch ERR_STRUCT — targeting disarmed to vanilla");
                return;
            }
            if (rc == ERR_RANGE) {
                return; // параметр-промах: весь тик свипом, эпоха ретраится на следующем тике
            }
            SNAPSHOT_EMPTY = false;
            SNAPSHOT = arr;         // публикуем снапшот ДО колонки
            NEAREST = col;
            NEAREST_LEN = rc;       // volatile write = publication edge для читателей
            EPOCH_TICK = t;         // release-edge: читатели видят консистентную тройку
            if (!EPOCH_LOGGED) {
                EPOCH_LOGGED = true;
                LOG.info("[crussty-plugin] " + LABEL + ": sense epoch ok tick=" + t
                        + " senseSlots=" + rc + " players=" + arr.length
                        + " (bulk JNI 1/tick over soa population)");
            }
        }
    }

    // ------------------------------------------------------------------
    // Decision core + exhaustive oracle (selfTest): accelerated pick vs
    // vanilla ladder on synthetic cases. The entity path
    // (nearestEntityGateImpl) is a rule-for-rule transcription of corePick.
    // ------------------------------------------------------------------

    /** Ванильная лестница (javap 0..97) над синтетикой — эталон оракула. */
    static int naivePick(double[] d, boolean[] pass) {
        double bestD = -1.0;
        int best = -1;
        for (int i = 0; i < d.length; i++) {
            if (!pass[i]) {
                continue;
            }
            if (bestD == -1.0 || d[i] < bestD) {
                bestD = d[i];
                best = i;
            }
        }
        return best;
    }

    /**
     * Ускоренное ядро: R (индекс idxR, результат теста rPass) устанавливается
     * первым, затем полный свип с decision-exact prune/tie-правилами. Возвращает
     * индекс пика или -1. rPass обязан равняться pass[idxR] при idxR >= 0
     * (тест чист) — противоречивые комбинации оракулом не подаются.
     */
    static int corePick(double[] d, boolean[] pass, int idxR, boolean rPass) {
        int n = d.length;
        double bestD = -1.0;
        int best = -1;
        int idxBest = -1;
        boolean rFailed = false;
        if (idxR >= 0 && idxR < n) {
            if (rPass) {
                bestD = d[idxR];
                best = idxR;
                idxBest = idxR;
            } else {
                rFailed = true;
            }
        }
        for (int i = 0; i < n; i++) {
            if (best >= 0) {
                if (i == idxBest) {
                    continue;
                }
                double di = d[i];
                if (di > bestD) {
                    continue; // строго дальше — не может выиграть строгую лестницу
                }
                if (di == bestD && i > idxBest) {
                    continue; // связка ПОЗЖЕ best — ваниль хранит более ранний
                }
                if (!pass[i]) {
                    continue;
                }
                if (di < bestD) {
                    bestD = di;
                    best = i;
                    idxBest = i;
                } else if (di == bestD && i < idxBest) {
                    best = i; // связка РАНЬШЕ — ваниль отдала бы её (keep-earlier)
                    idxBest = i;
                }
            } else {
                boolean p = pass[i];
                if (rFailed && i == idxR) {
                    p = false; // R уже оттестирован однажды (чистый предикат)
                }
                if (!p) {
                    continue;
                }
                bestD = d[i];
                best = i;
                idxBest = i;
            }
        }
        return best;
    }

    /**
     * Exhaustive-оракул: для n ≤ 4, дистанций из {1,2,3,4}, всех паттернов
     * pass и всех позиций R — corePick == naivePick. Единственный источник
     * несовпадения мог бы быть в tie/prune-правилах — они покрыты полностью.
     */
    static boolean decisionCoreOracle() {
        int[] distSet = {1, 2, 3, 4};
        for (int n = 0; n <= 4; n++) {
            int combos = 1;
            for (int i = 0; i < n; i++) {
                combos *= distSet.length;
            }
            for (int combo = 0; combo < combos; combo++) {
                double[] d = new double[n];
                int c = combo;
                for (int i = 0; i < n; i++) {
                    d[i] = distSet[c % distSet.length];
                    c /= distSet.length;
                }
                for (int pm = 0; pm < (1 << n); pm++) {
                    boolean[] pass = new boolean[n];
                    for (int i = 0; i < n; i++) {
                        pass[i] = ((pm >> i) & 1) != 0;
                    }
                    int naive = naivePick(d, pass);
                    for (int idxR = -1; idxR < n; idxR++) {
                        for (int rp = 0; rp < 2; rp++) {
                            if (idxR >= 0 && pass[idxR] != (rp == 1)) {
                                continue; // тест чист: rPass == pass[idxR]
                            }
                            int acc = corePick(d, pass, idxR, rp == 1);
                            if (acc != naive) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        return true;
    }

    /**
     * selfTest — вызывается rust-стороной на ТОЛЬКО ЧТО определённом классе ДО
     * ARM (TASK-437-A pattern; урок inside2-NCDFE: никаких static-native в
     * &lt;clinit&gt;, первый active use = сам selfTest после RegisterNatives).
     * false → дизарм навсегда (fail-closed, ваниль бит-в-байт).
     */
    public static boolean selfTest() {
        try {
            if (!probeOnce()) {
                return false; // senseProbe magic-mismatch/throw
            }
            return decisionCoreOracle();
        } catch (Throwable t) {
            return false;
        }
    }
}
