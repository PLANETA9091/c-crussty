package net.minecraft.world.entity.ai.goal;

import java.lang.reflect.Field;
import java.util.Iterator;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.logging.Logger;

import net.minecraft.server.MinecraftServer;

/**
 * GOAL-SELECTOR BATCH (TASK-414-C2, pathfinder/brain виток-2 — lever
 * cmp416_gsel3): batch-плоскость тика GoalSelector (Mob.goalSelector +
 * Mob.targetSelector; recon round-cvs2v2: GoalSelector.tick subtree = 9.54%
 * wall, из них scheduler-bookkeeping ≈ 2.9% — linked-set iteration +
 * lockedFlags map + removeIf + flag-сет-операции; neighbor-batch/NodeEvaluator
 * = 1.51-1.65% &lt; 2% порога — таргет смещён по протоколу).
 *
 * ДОСТАВКА: rust-стадия ретаргетит РОВНО 4 сайта invokevirtual
 * {@code GoalSelector.tick()V} в {@code Mob.serverAiStep} (×2) и
 * {@code Mob.inactiveTick} (×2) на статический мост
 * {@code GoalBatchOps.tickGate(GoalSelector)V} (receiver-prepended desc,
 * contract retarget_virtual_to_static — stack-shape идентичен). STRICT census:
 * сайтов tick()V в ядре ровно 4 (javap ground truth Mob/serverAiStep+inactiveTick);
 * чужие классы не трогаются; класс GoalSelector сам не патчится.
 *
 * ПЛОСКОСТЬ (закон 6 RUST-FIRST): rust-плоскость goal_batch держит SoA реестр
 * селекторов (selId → flags/priorities коло́нки, синхронизируется bulk-JNI
 * {@code gselRegister} при редких перестройках зеркала) и получает ОДИН bulk
 * JNI за серверный тик {@code gselEpoch(tick, n, selIds, locks, disabled,
 * stats)} (DOD-проход по реестру + телеметрия) — ZERO per-entity JNI.
 *
 * РЕШЕНИЯ — ВАНИЛЬ БИТ-В-БАЙТ: последовательный двухфазный цикл ванильного
 * GoalSelector.tick воспроизведён над плоским зеркалом (WrappedGoal[] в
 * порядке линкованного множества, int[8] lock-plane по ordinal'ам Goal.Flag):
 *
 *   - goalCleanup: {@code isRunning && ((flags & disabled) != 0 ||
 *     !canContinueToUse()) → stop()} — те же вызовы в том же порядке;
 *   - removeIf(lockedFlags) ≙ очистка lock-plane битов owner'ов с
 *     isRunning()==false (owner читается как ваниль — свежее поле);
 *   - goalUpdate: {@code !isRunning && (flags & disabled)==0 &&
 *     replaceable(flags) && canUse() → lock(flags) → start()} — canUse/start
 *     вызываются в ванильном порядке, лок-состояние последовательно
 *     учитывается в replaceable (как ванильная goalCanBeReplacedForAllFlags
 *     поверх свежей карты).
 *
 * ЗЕРКАЛО: {@code goals[]} снапшотится из {@code getAvailableGoals()} при
 * первом sighting селектора (addGoal/removeGoal редки; порядок линкованного
 * множества между мутациями стабилен — дубликаты не двигают порядок).
 * disabled-маска читается из приватного поля {@code goalTypes} через
 * кэшированный reflection-Field (1 чтение на гейт; setControlFlag мутации
 * видны немедленно). Ноль вложенных классов (S7-163) — состояние в плоских
 * статических картах по selId.
 *
 * FAIL-CLOSED: пустой/чужой CRUSSTY_LEVER_FLAG → сайт не ретаргетится
 * rust-стороной вообще (ваниль бит-в-байт); gselProbe magic-mismatch /
 * gselEpoch ERR_STRUCT → epoch-плоскость дизармится навсегда (телеметрия
 * уходит, решения и так всегда были java-ваниль); пустой реестр без
 * возможности перестройки → ванильный gs.tick() на этот вызов.
 */
public final class GoalBatchOps {

    private static final Logger LOG = Logger.getLogger("GoalBatchOps");

    static final String LEVER_FLAG = "cmp416_gsel3";
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;
    private static final int BATCH_CAP = 1 << 16;

    // ---- gate ----

    static boolean leverEnabled() {
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

    // ---- flat per-selector mirror по selId (S7-163: ноль вложенных классов) ----

    /** selector → stable dense id (identity semantics). */
    static final Map<GoalSelector, Integer> SEL_IDS = new ConcurrentHashMap<>();
    /** selId → goal snapshot в ванильном (linked) порядке. */
    static final Map<Integer, WrappedGoal[]> MIR_GOALS = new ConcurrentHashMap<>();
    /** selId → lock-plane: per flag ordinal → owner idx, -1 = свободен. */
    static final Map<Integer, int[]> MIR_LOCKS = new ConcurrentHashMap<>();
    static final AtomicInteger NEXT_SEL_ID = new AtomicInteger(1);
    static final int FLAG_COUNT = Math.min(8, Goal.Flag.values().length);

    // ---- per-tick batch (ONE bulk JNI per server tick; batch guard lock) ----

    static final Object BATCH_LOCK = new Object();
    static int batchTick = -1;
    static int batchCount = 0;
    static int[] batchSel = new int[1024];
    static long[] batchLocks = new long[1024];
    static long[] batchDisabled = new long[1024];
    static int[] batchStats = new int[1024];

    // ---- telemetry (effect markers) ----

    static final AtomicLong EPOCHS = new AtomicLong();
    static final AtomicLong GATES = new AtomicLong();
    static final AtomicLong REBUILDS = new AtomicLong();
    static final AtomicLong VANILLA_FALLBACKS = new AtomicLong();
    static volatile boolean epochAlive = true;
    static volatile long markerLogTick = 0;

    static native int gselProbe();
    static native int gselRegister(int selId, int n, long[] flags, int[] priorities);
    static native int gselEpoch(int tick, int n, int[] selIds, long[] locks,
                                long[] disabled, int[] stats);

    // ------------------------------------------------------------------
    // GATE — единственная точка входа (4 ретаргетнутых сайта Mob)
    // ------------------------------------------------------------------

    public static void tickGate(GoalSelector gs) {
        if (!leverEnabled() || GOAL_TYPES_FIELD == null || !epochAlive) {
            gs.tick();
            return;
        }
        try {
            tickGateInner(gs);
        } catch (Throwable t) {
            // fail-closed: любой сбой моста = ваниль на этот вызов
            VANILLA_FALLBACKS.incrementAndGet();
            gs.tick();
        }
    }

    private static void tickGateInner(GoalSelector gs) {
        final int tick = (int) MinecraftServer.getServer().getTickCount();
        long disabled;
        Object oset;
        try {
            oset = GOAL_TYPES_FIELD.get(gs);
        } catch (IllegalAccessException e) {
            gs.tick();
            return;
        }
        if (oset == null) {
            gs.tick();
            return;
        }
        @SuppressWarnings("unchecked")
        ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet<Goal.Flag> types =
                (ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet<Goal.Flag>) oset;
        disabled = types.getBackingSet();

        Integer sidBox = SEL_IDS.get(gs);
        if (sidBox == null) {
            synchronized (BATCH_LOCK) {
                sidBox = SEL_IDS.get(gs);
                if (sidBox == null) {
                    int id = NEXT_SEL_ID.getAndIncrement();
                    SEL_IDS.put(gs, id);
                    MIR_LOCKS.put(id, new int[FLAG_COUNT]);
                    sidBox = id;
                }
            }
        }
        final int selId = sidBox;

        WrappedGoal[] goals = MIR_GOALS.get(selId);
        if (goals == null) {
            synchronized (BATCH_LOCK) {
                if (tick != batchTick) {
                    flushBatch();
                    batchTick = tick;
                    batchCount = 0;
                }
                goals = tryRebuild(selId, gs);
            }
            if (goals == null) {
                // реестр недоступен (SoA sync dead) — ваниль на этот вызов
                VANILLA_FALLBACKS.incrementAndGet();
                gs.tick();
                return;
            }
        }

        synchronized (BATCH_LOCK) {
            if (tick != batchTick) {
                flushBatch();
                batchTick = tick;
                batchCount = 0;
            }
            if (batchCount >= batchSel.length && batchSel.length < BATCH_CAP) {
                int cap = Math.min(BATCH_CAP, batchSel.length * 2);
                batchSel = java.util.Arrays.copyOf(batchSel, cap);
                batchLocks = java.util.Arrays.copyOf(batchLocks, cap);
                batchDisabled = java.util.Arrays.copyOf(batchDisabled, cap);
                batchStats = java.util.Arrays.copyOf(batchStats, cap);
            }
            if (batchCount < batchSel.length) {
                int[] lockOwner = MIR_LOCKS.get(selId);
                long locks = 0;
                for (int b = 0; b < lockOwner.length; b++) {
                    int owner = lockOwner[b];
                    if (owner >= 0 && owner < goals.length && goals[owner] != null
                            && goals[owner].isRunning()) {
                        locks |= (1L << b);
                    }
                }
                batchSel[batchCount] = selId;
                batchLocks[batchCount] = locks;
                batchDisabled[batchCount] = disabled;
                batchCount++;
            }
        }
        GATES.incrementAndGet();
        logMarker(tick);
        runVanillaScheduler(goals, MIR_LOCKS.get(selId), disabled);
    }

    /** ONE bulk JNI за тик: DOD-проход rust-реестра + телеметрия. */
    private static void flushBatch() {
        if (batchCount <= 0) {
            return;
        }
        int rc = gselEpoch(batchTick, batchCount, batchSel, batchLocks,
                batchDisabled, batchStats);
        if (rc < 0 && rc != ERR_RANGE) {
            epochAlive = false; // структурный сбой — телеметрия дизарм, решения уже java-ваниль
            LOG.warning("[gsel] gselEpoch rc=" + rc
                    + " — epoch plane disarmed (schedulers stay vanilla-exact)");
        } else if (rc >= 0) {
            EPOCHS.incrementAndGet();
        }
    }

    /** Зеркальный снапшот: getAvailableGoals + rust SoA sync (bulk, rare). */
    private static WrappedGoal[] tryRebuild(int selId, GoalSelector gs) {
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
        int rc = gselRegister(selId, n, flags, prios);
        if (rc < 0 && rc != ERR_RANGE) {
            return null; // SoA sync недоступен — плоскость не armed
        }
        int[] lockOwner = new int[FLAG_COUNT];
        java.util.Arrays.fill(lockOwner, -1);
        MIR_LOCKS.put(selId, lockOwner);
        MIR_GOALS.put(selId, goals);
        REBUILDS.incrementAndGet();
        return goals;
    }

    /**
     * Ванильный GoalSelector.tick над плоским зеркалом — порядок и решения
     * бит-в-байт (см. javap GoalSelector.tick / goalCanBeReplacedForAllFlags).
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
    private static void logMarker(int tick) {
        long ep = EPOCHS.get();
        if (ep <= 6 || tick - markerLogTick >= 1200) {
            markerLogTick = tick;
            LOG.info("[gsel] epoch t=" + tick + " epochs=" + ep + " gates=" + GATES.get()
                    + " selectors=" + SEL_IDS.size() + " rebuilds=" + REBUILDS.get()
                    + " vanillaFallbacks=" + VANILLA_FALLBACKS.get()
                    + " epochAlive=" + epochAlive);
        }
    }

    private GoalBatchOps() {
    }
}
