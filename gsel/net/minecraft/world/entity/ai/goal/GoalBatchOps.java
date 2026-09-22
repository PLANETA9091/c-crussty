package net.minecraft.world.entity.ai.goal;

import java.lang.reflect.Field;
import java.util.Iterator;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.LongAdder;
import java.util.logging.Logger;

import net.minecraft.server.MinecraftServer;

/**
 * GOAL-SELECTOR BATCH (TASK-414-C2, iter-4 FASTPATH — lever cmp416_gsel3):
 * batch-плоскость тика GoalSelector (Mob.goalSelector + Mob.targetSelector).
 *
 * ИТЕРАЦИЯ-4 (TASK-416-B, бисекция по round-g3{a2,b,c} collapsed-профилям):
 * iter-3 дал nav_ai −2.3..−2.7пп, но маржинал был отрицателен — батч-цикл
 * платил за гейт (×~300k/тик на 5 тик-потоках):
 *
 *   1. {@code System.getenv} НА КАЖДОМ гейте (leverEnabled) — byte[] encode
 *      + Variable boxing = мусор (15 alloc-сэмплов);
 *   2. Integer-боксинг selId (&gt;127 = вне кэша valueOf) в двух CHM.get
 *      на гейт (23 alloc-сэмпла) — ×600k/тик = главный GC-churn (young
 *      1129 мелких → 105-119 крупных, avg-pause 156мс vs cvs 22-24мс);
 *   3. глобальный {@code BATCH_LOCK} на аппенд-батч + JNI-flush внутри
 *      лока — monitor-park 42 wall-сэмпла (контеншн region-workers);
 *   4. {@code AtomicLong.get} телеметрии + tickCount + 3-я CHM.get на гейт.
 *
 * ИТЕРАЦИЯ-4 УСТРАНЯЕТ ВСЁ ЧЕТЫРЕ (zero per-gate garbage, zero hot-path
 * monitor):
 *
 *   - lever-флаг кэшируется в {@code static final} (env процесса
 *     фиксирован до старта JVM — семантика STRICT-eq неизменна);
 *   - состояние селектора = ОДНА identity-CHM {@code STATE.get(gs)} →
 *     {@code Object[]{WrappedGoal[], int[FLAG_COUNT]}} (создаётся один раз
 *     при первом sighting; ноль боксинга — ключ по identity, значения —
 *     плоские массивы);
 *   - горячий путь БЕЗ локов: lock-plane каждого селектора трогает ровно
 *     один region-поток (моб живёт в одном регионе); редкий путь
 *     регистрации — отдельный {@code REG_LOCK};
 *   - epoch = ОДИН bulk-JNI за серверный тик, СКАЛЯРНЫЙ
 *     {@code gselEpoch(tick, gatesSum)} — ноль JNI-копий массивов
 *     (65k×3 GetArrayRegion/тик ушли), границы тика детектируются
 *     volatile-чтением {@code epochTickSeen} + короткий {@code EPOCH_LOCK}
 *     только на пересечении границы (≤1 раз/тик);
 *   - телеметрия гейтов = {@code LongAdder} (var-rectified, без CAS
 *     ping-pong между 5 тик-потоками).
 *
 * ПЛОСКОСТЬ (закон 6 RUST-FIRST): rust-сторона (goal_batch.rs) держит
 * DENSE SoA-реестр (goals_col/flags_col/prios_col + offsets — коло́нки,
 * selId = индекс), синхронизируемый bulk-JNI {@code gselRegister} при
 * редкой регистрации; {@code gselEpoch} за тик делает DOD-проход по
 * коло́нкам (сумма целей, слоты) — data-residency + телеметрия, ZERO
 * per-entity JNI.
 *
 * РЕШЕНИЯ — ВАНИЛЬ БИТ-В-БАЙТ (не менялись с iter-3): последовательный
 * двухфазный цикл ванильного GoalSelector.tick над плоским зеркалом:
 *
 *   - goalCleanup: {@code isRunning && ((flags & disabled) != 0 ||
 *     !canContinueToUse()) → stop()} — те же вызовы в том же порядке;
 *   - removeIf(lockedFlags) ≙ очистка lock-plane битов owner'ов с
 *     isRunning()==false;
 *   - goalUpdate: {@code !isRunning && (flags & disabled)==0 &&
 *     replaceable(flags) && canUse() → lock(flags) → start()} — canUse/
 *     start в ванильном порядке, replaceable поверх живого lock-plane.
 *
 * ЗЕРКАЛО: {@code goals[]} снапшотится из {@code getAvailableGoals()} при
 * первом sighting (addGoal/removeGoal редки; порядок стабилен — как в
 * iter-3). disabled-маска читается СВЕЖЕЙ на каждом гейте из приватного
 * {@code goalTypes} через кэшированный reflection-Field (мутации
 * setControlFlag видны немедленно — бит-в-байт с ванилью).
 *
 * FAIL-CLOSED: пустой/чужой CRUSSTY_LEVER_FLAG → сайт не ретаргетится
 * rust-стороной (ваниль бит-в-байт); gselProbe magic-mismatch /
 * gselEpoch ERR_STRUCT → epoch-плоскость дизармится навсегда (решения и
 * так всегда java-ваниль); любой Throwable на гейте → ванильный
 * {@code gs.tick()} на этот вызов.
 */
public final class GoalBatchOps {

    private static final Logger LOG = Logger.getLogger("GoalBatchOps");

    static final String LEVER_FLAG = "cmp416_gsel3";
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    // ---- gate (кэш на класс-инициализации: env фиксирован до старта JVM) ----

    static final boolean LEVER_ON = computeLeverOn();

    private static boolean computeLeverOn() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals(LEVER_FLAG);
    }

    // ---- disabled-flags reflection (GoalSelector.goalTypes приватен) ----

    private static final Field GOAL_TYPES_FIELD;

    static {
        Field f = null;
        try {
            f = GoalSelector.class.getDeclaredField("goalTypes");
            f.setAccessible(true);
        } catch (Throwable t) {
            LOG.warning("[gsel] goalTypes reflection unavailable: " + t);
            f = null;
        }
        GOAL_TYPES_FIELD = f;
    }

    // ---- per-selector state: ОДНА identity-CHM, ОДИН lookup на гейт ----
    // st[0] = WrappedGoal[] (ванильный linked-порядок), st[1] = int[FLAG_COUNT]
    // lock-plane (owner idx per flag ordinal, -1 = свободен). Проверка
    // S7-163 (см. rust-тест): GoalBatchOps = ОДИН classfile — Object[] + плоские
    // массивы вместо вложенного state-класса.

    static final Map<GoalSelector, Object[]> STATE = new ConcurrentHashMap<>();

    static final int FLAG_COUNT = Math.min(8, Goal.Flag.values().length);

    // ---- epoch boundary (ОДИН скалярный bulk-JNI за серверный тик) ----

    static final Object EPOCH_LOCK = new Object();
    /** последний тик, для которого epoch уже отработал (volatile: hot-read). */
    static volatile long epochTickSeen = -1L;
    /** редкий путь регистрации первого sighting селектора. */
    static final Object REG_LOCK = new Object();
    static int NEXT_SEL_ID = 0;

    // ---- telemetry (effect markers; гейты — LongAdder: 5 тик-потоков) ----

    static final LongAdder GATES = new LongAdder();
    static final AtomicLong EPOCHS = new AtomicLong();
    static final AtomicLong REBUILDS = new AtomicLong();
    static final AtomicLong VANILLA_FALLBACKS = new AtomicLong();
    static volatile boolean epochAlive = true;
    static volatile long markerLogTick = 0;

    static native int gselProbe();
    static native int gselRegister(int selId, int n, long[] flags, int[] priorities);
    /** iter-4: скалярный epoch — ноль JNI-копий массивов (бисекция g3b). */
    static native int gselEpoch(int tick, long gatesSum);

    // ------------------------------------------------------------------
    // GATE — единственная точка входа (4 ретаргетнутых сайта Mob)
    // ------------------------------------------------------------------

    public static void tickGate(GoalSelector gs) {
        if (!LEVER_ON || GOAL_TYPES_FIELD == null || !epochAlive) {
            gs.tick();
            return;
        }
        try {
            Object[] st = STATE.get(gs);
            if (st == null) {
                st = registerSlow(gs);
                if (st == null) {
                    // реестр недоступен (SoA sync dead) — ваниль на этот вызов
                    VANILLA_FALLBACKS.incrementAndGet();
                    gs.tick();
                    return;
                }
            }
            long disabled = readDisabled(gs);
            if (disabled == -1L) {
                // недоступная disabled-маска — ваниль на этот вызов (как iter-3)
                gs.tick();
                return;
            }
            maybeEpoch();
            GATES.increment();
            runVanillaScheduler((WrappedGoal[]) st[0], (int[]) st[1], disabled);
        } catch (Throwable t) {
            // fail-closed: любой сбой моста = ваниль на этот вызов
            VANILLA_FALLBACKS.incrementAndGet();
            gs.tick();
        }
    }

    /** Свежая disabled-маска (setControlFlag-мутации видны немедленно). */
    private static long readDisabled(GoalSelector gs) throws IllegalAccessException {
        Object oset = GOAL_TYPES_FIELD.get(gs);
        if (oset == null) {
            return -1L; // невалидная маска → ваниль-фолбэк (см. tickGate)
        }
        @SuppressWarnings("unchecked")
        ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet<Goal.Flag> types =
                (ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet<Goal.Flag>) oset;
        long m = types.getBackingSet();
        // ≤8 флагов ⇒ валидная маска только в младших 64-FLAG_COUNT..0 битах;
        // -1 (все 64) не может быть легитимной — sentinel-безопасно
        return m;
    }

    /**
     * iter-4: границы тика = volatile-чтение + (на пересечении) короткий
     * EPOCH_LOCK — ровно один bulk-JNI за тик, вне горячих путей остальных
     * гейтов того же тика (после volatile-записи они уходят на fast-skip).
     */
    private static void maybeEpoch() {
        final long t = MinecraftServer.getServer().getTickCount();
        if (t == epochTickSeen) {
            return;
        }
        synchronized (EPOCH_LOCK) {
            if (t == epochTickSeen) {
                return;
            }
            epochTickSeen = t;
        }
        int rc = gselEpoch((int) t, GATES.sum());
        if (rc < 0 && rc != ERR_RANGE) {
            epochAlive = false; // структурный сбой — телеметрия дизарм, решения уже java-ваниль
            LOG.warning("[gsel] gselEpoch rc=" + rc
                    + " — epoch plane disarmed (schedulers stay vanilla-exact)");
        } else if (rc >= 0) {
            EPOCHS.incrementAndGet();
            logMarker(t);
        }
    }

    /** Редкий путь: первый sighting селектора — SoA sync + зеркало + lock-plane. */
    private static Object[] registerSlow(GoalSelector gs) {
        synchronized (REG_LOCK) {
            Object[] existing = STATE.get(gs);
            if (existing != null) {
                return existing; // double-check (другой region-поток зарегистрировал)
            }
            Set<WrappedGoal> live = gs.getAvailableGoals();
            if (live == null) {
                return null;
            }
            int n = live.size();
            WrappedGoal[] goals = new WrappedGoal[n];
            long[] flags = new long[n];
            int[] prios = new int[n];
            Iterator<WrappedGoal> it = live.iterator();
            int i = 0;
            while (it.hasNext()) {
                WrappedGoal g = it.next();
                goals[i] = g;
                flags[i] = g.getFlags().getBackingSet();
                prios[i] = g.getPriority();
                i++;
            }
            int selId = NEXT_SEL_ID++;
            int rc = gselRegister(selId, n, flags, prios);
            if (rc < 0 && rc != ERR_RANGE) {
                return null; // SoA sync недоступен — плоскость не armed
            }
            int[] lockOwner = new int[FLAG_COUNT];
            java.util.Arrays.fill(lockOwner, -1);
            Object[] st = new Object[] { goals, lockOwner };
            STATE.put(gs, st);
            REBUILDS.incrementAndGet();
            return st;
        }
    }

    /**
     * Ванильный GoalSelector.tick над плоским зеркалом — порядок и решения
     * бит-в-байт (см. javap GoalSelector.tick / goalCanBeReplacedForAllFlags).
     * НЕ менялся с iter-3 (это и есть nav_ai-эффект −2.3..−2.7пп).
     */
    private static void runVanillaScheduler(WrappedGoal[] goals, int[] lockOwner,
                                            long disabled) {
        final int n = goals.length;

        // goalCleanup — ванильный порядок, stop() без canContinue для
        // flag-blocked (как ваниль), canContinueToUse() только для остальных.
        for (int i = 0; i < n; i++) {
            WrappedGoal g = goals[i];
            if (g.isRunning()
                    && (((g.getFlags().getBackingSet() & disabled) != 0)
                        || !g.canContinueToUse())) {
                g.stop();
            }
        }
        // removeIf(lockedFlags, !value.isRunning()) ≙ lock-plane sweep
        for (int b = 0; b < lockOwner.length; b++) {
            int owner = lockOwner[b];
            if (owner >= 0) {
                WrappedGoal og = (owner < n) ? goals[owner] : null;
                if (og == null || !og.isRunning()) {
                    lockOwner[b] = -1;
                }
            }
        }
        // goalUpdate — последовательный проход с живым lock-состоянием
        for (int i = 0; i < n; i++) {
            WrappedGoal g = goals[i];
            if (g.isRunning()) {
                continue;
            }
            long fl = g.getFlags().getBackingSet();
            if ((fl & disabled) != 0) {
                continue;
            }
            if (!replaceable(goals, lockOwner, g, fl)) {
                continue;
            }
            if (!g.canUse()) {
                continue;
            }
            long bits = fl;
            while (bits != 0) {
                int b = Long.numberOfTrailingZeros(bits);
                bits &= bits - 1;
                if (b < lockOwner.length) {
                    lockOwner[b] = i;
                }
            }
            g.start();
        }
    }

    /** Ваниль goalCanBeReplacedForAllFlags над lock-plane. */
    private static boolean replaceable(WrappedGoal[] goals, int[] lockOwner,
                                       WrappedGoal candidate, long flags) {
        long bits = flags;
        while (bits != 0) {
            int b = Long.numberOfTrailingZeros(bits);
            bits &= bits - 1;
            if (b >= lockOwner.length) {
                break;
            }
            int owner = lockOwner[b];
            if (owner >= 0 && owner < goals.length) {
                WrappedGoal locked = goals[owner];
                if (locked != null && !locked.canBeReplacedBy(candidate)) {
                    return false;
                }
            }
        }
        return true;
    }

    /** ЭФФЕКТ-МАРКЕР: первые 6 эпох + каждые ~1200 тиков — grep-доказательство жизни. */
    private static void logMarker(long tick) {
        long ep = EPOCHS.get();
        if (ep <= 6 || tick - markerLogTick >= 1200) {
            markerLogTick = tick;
            LOG.info("[gsel] epoch t=" + tick + " epochs=" + ep + " gates=" + GATES.sum()
                    + " selectors=" + STATE.size() + " rebuilds=" + REBUILDS.get()
                    + " vanillaFallbacks=" + VANILLA_FALLBACKS.get()
                    + " epochAlive=" + epochAlive);
        }
    }

    private GoalBatchOps() {
    }
}
