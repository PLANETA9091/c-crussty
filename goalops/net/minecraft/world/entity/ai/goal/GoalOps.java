package net.minecraft.world.entity.ai.goal;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandles.Lookup;
import java.lang.reflect.Field;
import java.util.Map;
import java.util.Set;
import java.util.logging.Logger;

import net.minecraft.util.profiling.Profiler;
import net.minecraft.util.profiling.ProfilerFiller;
import net.minecraft.world.entity.ai.goal.Goal.Flag;
import ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet;

/**
 * GOAL-SELECTOR FLAT PRIORITY FAST-PATH (TASK-421-A brain-slice, lever
 * cmp421_brain; закон 6: подсистема sense+brain целиком на rust-носителе —
 * этот мост = срез «goal priorities», вайринг/ретаргет — в крейте
 * src/goal_selector.rs).
 *
 * ЗАМЕНА сайта {@code invokevirtual GoalSelector.tick()V} внутри
 * {@code Mob.serverAiStep()V} (purpur-1.21.10 javap ground truth: РОВНО 2
 * сайта — targetSelector + goalSelector; сайты tickRunningGoals(Z) НЕ
 * трогаются — там нет выигрыша). Тело {@code GoalSelector.tick()} из
 * ванили (javap 1:1): фазы goalCleanup → lockedFlags purge → goalUpdate →
 * goalTick (tickRunningGoals(true)), с ТЕМИ ЖЕ вызовами canContinueToUse /
 * canUse / start / stop / tick в ТОМ ЖЕ insertion-order (ванильный порядок
 * итерации доступных целей).
 *
 * TASK-422-B (iter-2): слайс ЦЕЛИКОМ — сайт tick()V (чётные тики) ДОПОЛНЕН
 * сайтами tickRunningGoals(Z) (нечётные тики, Paper-сплит: РОВНО 2 сайта
 * в serverAiStep, оба false) → {@link #tickRunningGate} — та же плоская
 * семантика одним обходом. STRICT-OR гейт: cmp421_brain ИЛИ cmp422_brain2
 * (носитель-унион + вектор-ноги тика-422; пустой/чужой флаг = ваниль).
 *
 * ФЛЕТ-ФАСТПАТ (единственное отличие от ванили — устройство данных): три
 * полных обхода ObjectLinkedOpenHashSet (итератор: hasNext/next + checkcast
 * на каждом шаге) заменяются ОДНИМ обходом с построением плоского
 * WrappedGoal[]-скретча (ThreadLocal, без аллокации после прогрева) и
 * плоскими проходами по массиву. ЧЛЕНСТВО множества в ядре не мутирует
 * внутри tick (addGoal/removeGoal зовутся из registerGoals ВНЕ тика;
 * stop()/start() флипают только isRunning-поле WrappedGoal, которое
 * проходы читают ЖИВЫМ на каждом шаге — как ванильный третий обход).
 * isRunning-полуготовность между проходами = та же семантика ванили.
 *
 * lockedFlags (EnumMap) трогается ТОЛЬКО Map-операциями (get/put/remove по
 * Goal.Flag.values() — порядок ординалов = порядок EnumMap-итерации
 * ванильного removeIf). NO_GOAL-сентинел ванили (GoalSelector$2:
 * isRunning()=false всегда; goal GoalSelector$1: canUse()=false) сворачивается
 * в null-проверки, эквивалентные по javap:
 *   - getOrDefault(f, NO_GOAL).stop() на отсутствующем = noop
 *     (WrappedGoal.stop при isRunning=false — javap WrappedGoal.stop);
 *   - getOrDefault(f, NO_GOAL).canBeReplacedBy(g) на отсутствующем = true
 *     (Goal.isInterruptable()=true, NO_GOAL.priority=Integer.MAX_VALUE —
 *     javap GoalSelector$1/$2).
 *
 * FAIL-CLOSED: STRICT-eq CRUSSTY_LEVER_FLAG ∈ {"cmp421_brain",
 * "cmp422_brain2"} (пустой/чужой флаг — rust вообще не ретаргетит сайт, а
 * гейт зовёт ванильное тело);
 * любое reflect-отражение/структурный дрейф на SETUP-фазе (ДО мутаций) →
 * ванильный sel.tick() на этот вызов; исключение в МУТАЦИОННОЙ фазе
 * распространяется как в ванили (тот же частичный state — паритет).
 */
public final class GoalOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // STRICT-OR (TASK-422-B): носитель-флаг cmp421_brain ИЛИ вектор-флаг
        // cmp422_brain2; пустой/чужой флаг = ваниль бит-в-байт.
        return f != null && (f.trim().equals("cmp421_brain")
                || f.trim().equals("cmp422_brain2")
                // TASK-424-A: GC-ревизия brain3 (STRICT OR).
                || f.trim().equals("cmp423_brain3")
                // TASK-426-A: SoA-feed carrier (STRICT OR).
                || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp432_inside2") || f.trim().equals("cmp436_ins4")
                || f.trim().equals("cmp440_ins4d")
                || f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3") || f.trim().equals("cmp437_chunk4")
                || f.trim().equals("cmp443_mega") || f.trim().equals("cmp444_sensemega")); // TASK-443-B: mega-composition carrier STRICT OR
    }

    private static final boolean ENABLED = leverEnabled();
    private static volatile boolean broken = false;
    private static volatile boolean armLogged = false;
    private static volatile boolean runLogged = false;

    /** Активный флаг (для меток): cmp422_brain2 > cmp421_brain, иначе "(off)". */
    private static final String FLAG_LABEL = flagLabel();

    private static String flagLabel() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        if (f == null) {
            return "(off)";
        }
        return switch (f.trim()) {
            case "cmp422_brain2" -> "cmp422_brain2";
            case "cmp423_brain3" -> "cmp423_brain3";
            case "cmp424_mobfeed" -> "cmp424_mobfeed"; // TASK-426-A: SoA-feed carrier.
            case "cmp421_brain" -> "cmp421_brain";
            default -> "(off)";
        };
    }

    static final Logger LOG = Logger.getLogger("crussty-plugin");

    // ---- кешированный reflect-доступ на ЧТЕНИЕ к приватным полям
    //      GoalSelector (availableGoals/lockedFlags/goalTypes в ванили
    //      никогда не переприсваиваются; содержимое мутируется через
    //      публичные Map/Set-объекты) ----
    private static MethodHandle agGet;
    private static MethodHandle lfGet;
    private static MethodHandle gtGet;
    private static volatile boolean reflectReady = false;

    /** Goal.Flag.values() — порядок ординалов = порядок EnumMap-итерации. */
    private static final Flag[] FLAGS = Flag.values();

    /** ThreadLocal-скретч: [goals][len] одной парой (без второй TL-lookup'а). */
    private static final ThreadLocal<Object[]> SCRATCH =
            ThreadLocal.withInitial(() -> new Object[2]);

    private GoalOps() {}

    private static synchronized boolean reflectSetup() {
        if (reflectReady) {
            return true;
        }
        if (broken) {
            return false;
        }
        try {
            Lookup lk = MethodHandles.lookup();
            Field ag = GoalSelector.class.getDeclaredField("availableGoals");
            ag.setAccessible(true);
            Field lf = GoalSelector.class.getDeclaredField("lockedFlags");
            lf.setAccessible(true);
            Field gt = GoalSelector.class.getDeclaredField("goalTypes");
            gt.setAccessible(true);
            agGet = lk.unreflectGetter(ag);
            lfGet = lk.unreflectGetter(lf);
            gtGet = lk.unreflectGetter(gt);
            reflectReady = true;
            return true;
        } catch (Throwable t) {
            broken = true;
            LOG.warning("[crussty-plugin] cmp421_brain: goal-selector reflect setup failed — disarmed to vanilla: " + t);
            return false;
        }
    }

    /** Диагностика: armed ли флет-фастпат (для маркеров/тестов). */
    public static boolean armed() {
        return ENABLED && !broken && reflectReady;
    }

    /**
     * Замена сайта {@code invokevirtual GoalSelector.tick()V} в
     * Mob.serverAiStep. Stack-identical: receiver consummирован, desc =
     * virtual desc ()V с препендированным receiver-классом GoalSelector
     * (валидатор compose).
     */
    public static void tickGate(GoalSelector sel) {
        if (!ENABLED || broken) {
            sel.tick(); // ваниль (fail-closed)
            return;
        }
        Object[] scratch = SCRATCH.get();
        WrappedGoal[] arr;
        int n;
        Map<Flag, WrappedGoal> lf;
        OptimizedSmallEnumSet<Flag> disabled;
        try {
            // ---- SETUP-фаза: только чтения, ДО любых мутаций ----
            if (!reflectSetup()) {
                sel.tick();
                return;
            }
            Object setObj = agGet.invoke(sel);
            Object gtObj = gtGet.invoke(sel);
            Object lfObj = lfGet.invoke(sel);
            if (!(setObj instanceof Set<?> set)
                    || !(gtObj instanceof OptimizedSmallEnumSet)
                    || !(lfObj instanceof Map)) {
                sel.tick(); // структурный дрейф — ваниль этот вызов
                return;
            }
            @SuppressWarnings("unchecked")
            Map<Flag, WrappedGoal> lfMap = (Map<Flag, WrappedGoal>) lfObj;
            @SuppressWarnings("unchecked")
            OptimizedSmallEnumSet<Flag> gtSet = (OptimizedSmallEnumSet<Flag>) gtObj;
            lf = lfMap;
            disabled = gtSet;
            int size = set.size();
            if (size == 0) {
                sel.tick(); // пустой селектор — ваниль (нулевая работа)
                return;
            }
            Object arrObj = scratch[0];
            if (!(arrObj instanceof WrappedGoal[]) || ((WrappedGoal[]) arrObj).length < size) {
                arr = new WrappedGoal[Math.max(16, size * 2)];
                scratch[0] = arr;
            } else {
                arr = (WrappedGoal[]) arrObj;
            }
            int i = 0;
            for (Object o : set) {
                if (!(o instanceof WrappedGoal) || i >= arr.length) {
                    sel.tick(); // чужой контент / гонка роста — ваниль
                    return;
                }
                arr[i++] = (WrappedGoal) o;
            }
            if (i != size) {
                sel.tick(); // размер дрейфнул во время обхода — ваниль
                return;
            }
            n = i;
        } catch (Throwable t) {
            broken = true;
            LOG.warning("[crussty-plugin] cmp421_brain: goal-selector snapshot failed — disarmed to vanilla: " + t);
            sel.tick();
            return;
        }
        // ---- МУТАЦИОННАЯ фаза: точная ванильная последовательность
        //      (исключения целей распространяются как в ванили) ----
        final ProfilerFiller profiler = Profiler.get();
        profiler.push("goalCleanup");
        // ваниль (javap GoalSelector.tick 12-72):
        //   for (g : availableGoals)
        //     if (g.isRunning() && (goalContainsAnyFlags(g, disabled)
        //         || !g.canContinueToUse())) g.stop();
        // (javap 204-211: goalContainsAnyFlags ifne -> stop; canContinueToUse
        //  ifne -> skip; goalContainsAnyFlags = getFlags().hasCommonElements
        //  БЕЗ отрицания — javap GoalSelector 132-138 ireturn напрямую)
        for (int i = 0; i < n; i++) {
            WrappedGoal g = arr[i];
            if (g.isRunning()) {
                if (g.getFlags().hasCommonElements(disabled) || !g.canContinueToUse()) {
                    g.stop();
                }
            }
        }
        // ваниль (javap 73-92): lockedFlags.entrySet().removeIf(
        //   e -> !e.getValue().isRunning()) — EnumMap-итерация по ординалам
        for (Flag f : FLAGS) {
            WrappedGoal cur = lf.get(f);
            if (cur != null && !cur.isRunning()) {
                lf.remove(f);
            }
        }
        profiler.pop();
        profiler.push("goalUpdate");
        // ваниль (javap 107-271):
        //   for (g : availableGoals)
        //     if (!g.isRunning() && !goalContainsAnyFlags(g, disabled)
        //         && goalCanBeReplacedForAllFlags(g, lockedFlags) && g.canUse())
        for (int i = 0; i < n; i++) {
            WrappedGoal g = arr[i];
            if (g.isRunning()) {
                continue;
            }
            if (g.getFlags().hasCommonElements(disabled)) {
                continue;
            }
            if (!goalCanBeReplacedForAllFlags(g, lf)) {
                continue;
            }
            if (!g.canUse()) {
                continue;
            }
            // ваниль-блок блокировки флагов (javap 172-264)
            OptimizedSmallEnumSet<Flag> flags = g.getFlags();
            long l = flags.getBackingSet();
            int cnt = flags.size();
            for (int j = 0; j < cnt; j++) {
                Flag f = FLAGS[Long.numberOfTrailingZeros(l)];
                l ^= (l & -l); // IntegerUtil.getTrailingBit
                WrappedGoal prev = lf.get(f);
                // ваниль: getOrDefault(f, NO_GOAL).stop() — NO_GOAL.stop() =
                // noop (isRunning=false всегда) → null-проверка эквивалентна
                if (prev != null) {
                    prev.stop();
                }
                lf.put(f, g);
            }
            g.start();
        }
        profiler.pop();
        // ваниль (javap 277-281): tickRunningGoals(true) — финальный проход;
        // isRunning читается ЖИВЫМ (как ванильный третий обход множества);
        // параметр stopAllThreads=true → requiresUpdateEveryTick не важен
        for (int i = 0; i < n; i++) {
            WrappedGoal g = arr[i];
            if (g.isRunning()) {
                g.tick();
            }
        }
        if (!armLogged) {
            armLogged = true;
            LOG.info("[crussty-plugin] cmp421_brain: goal-selector EFFECT armed (first flat tick, goals="
                    + n + " — 1 set traversal + flat passes vs 3 vanilla traversals, exact stop/start order)");
        }
    }

    /**
     * Замена сайтов {@code invokevirtual GoalSelector.tickRunningGoals(Z)V}
     * в Mob.serverAiStep (нечётные тики, Paper-сплит; оба сайта передают
     * false). Тело ванили (javap GoalSelector.tickRunningGoals 1:1):
     * profiler push("goalTick") → для всех availableGoals:
     * isRunning() && (stopAll || requiresUpdateEveryTick()) → tick() → pop.
     * Единственное отличие — плоский проход по снапшоту вместо итератора
     * множества (тот же documented-стенс: addGoal/removeGoal вне тика;
     * isRunning читается ЖИВЫМ на каждом шаге, как ваниль).
     * Stack-identical: receiver consummирован, boolean остаётся вторым
     * аргументом (receiver-prepended static desc).
     */
    public static void tickRunningGate(GoalSelector sel, boolean stopAll) {
        if (!ENABLED || broken) {
            sel.tickRunningGoals(stopAll); // ваниль (fail-closed)
            return;
        }
        Object[] scratch = SCRATCH.get();
        WrappedGoal[] arr;
        int n;
        try {
            // ---- SETUP-фаза: только чтения, ДО любых мутаций ----
            if (!reflectSetup()) {
                sel.tickRunningGoals(stopAll);
                return;
            }
            Object setObj = agGet.invoke(sel);
            if (!(setObj instanceof Set<?> set)) {
                sel.tickRunningGoals(stopAll); // структурный дрейф — ваниль
                return;
            }
            int size = set.size();
            if (size == 0) {
                sel.tickRunningGoals(stopAll); // пустой селектор — ваниль
                return;
            }
            Object arrObj = scratch[0];
            if (!(arrObj instanceof WrappedGoal[]) || ((WrappedGoal[]) arrObj).length < size) {
                arr = new WrappedGoal[Math.max(16, size * 2)];
                scratch[0] = arr;
            } else {
                arr = (WrappedGoal[]) arrObj;
            }
            int i = 0;
            for (Object o : set) {
                if (!(o instanceof WrappedGoal) || i >= arr.length) {
                    sel.tickRunningGoals(stopAll); // чужой контент — ваниль
                    return;
                }
                arr[i++] = (WrappedGoal) o;
            }
            if (i != size) {
                sel.tickRunningGoals(stopAll); // дрейф размера — ваниль
                return;
            }
            n = i;
        } catch (Throwable t) {
            broken = true;
            LOG.warning("[crussty-plugin] " + label() + ": goal-selector running snapshot failed — disarmed to vanilla: " + t);
            sel.tickRunningGoals(stopAll);
            return;
        }
        // ---- МУТАЦИОННАЯ фаза: точная ванильная последовательность ----
        final ProfilerFiller profiler = Profiler.get();
        profiler.push("goalTick");
        for (int i = 0; i < n; i++) {
            WrappedGoal g = arr[i];
            if (g.isRunning() && (stopAll || g.requiresUpdateEveryTick())) {
                g.tick();
            }
        }
        profiler.pop();
        if (!runLogged) {
            runLogged = true;
            LOG.info("[crussty-plugin] " + label() + ": goal-selector running EFFECT armed (first flat tickRunning, goals="
                    + n + " stopAll=" + stopAll + " — 1 flat pass vs set-iterator traversal, exact isRunning/requiresUpdate order)");
        }
    }

    /** Метка активного флага для маркеров (STRICT-OR диагностика). */
    private static String label() {
        return FLAG_LABEL;
    }

    /** ваниль goalCanBeReplacedForAllFlags (javap 1:1; NO_GOAL свёрнут в null). */
    private static boolean goalCanBeReplacedForAllFlags(WrappedGoal wrapped,
            Map<Flag, WrappedGoal> lockedFlags) {
        OptimizedSmallEnumSet<Flag> flags = wrapped.getFlags();
        long l = flags.getBackingSet();
        int size = flags.size();
        for (int i = 0; i < size; i++) {
            Flag f = FLAGS[Long.numberOfTrailingZeros(l)];
            l ^= (l & -l);
            WrappedGoal cur = lockedFlags.get(f);
            if (cur != null && !cur.canBeReplacedBy(wrapped)) {
                return false;
            }
        }
        return true;
    }

}
