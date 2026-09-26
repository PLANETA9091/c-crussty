package net.minecraft.world.entity.ai.goal;

/**
 * GOAL-SELECTOR BRANCH-PREDICATE PROBE (ROUND-468 S88, lever
 * "cmp468_s88probe", WILD/ЛАБ RESEARCH-нога — НЕ оптимизация).
 *
 * ЧТО ЭТО: диагностический пасс-тру мост для измерения частот ИСХОДОВ
 * горячих бранчей goal-селектора на 150k-фикстуре. Ретаргетит ДВА сайта
 * (javap ground truth purpur-1.21.10 @ patched-kernel round-396-a):
 *
 *   1. {@code invokevirtual WrappedGoal.canUse()Z} внутри
 *      {@code GoalSelector.tick()V} — РОВНО 1 сайт (@ bc166, фаза goalUpdate)
 *      → {@link #canUseProbe(WrappedGoal)};
 *   2. {@code invokevirtual WrappedGoal.canBeReplacedBy(WrappedGoal)Z} внутри
 *      {@code GoalSelector.goalCanBeReplacedForAllFlags(WrappedGoal,Map)Z} —
 *      РОВНО 1 сайт (@ bc59) → {@link #canBeReplacedProbe(WrappedGoal,WrappedGoal)}.
 *
 * ВАНИЛЬНОСТЬ (закон 4): оба моста = чистый пасс-тру — вызывают ванильный
 * виртуальный метод и возвращают его результат БЕЗ обработки исключений
 * (throwable пробрасывается как в ванили), БЕЗ перестановки порядка
 * вычислений (обёртка добавляет только счётчики ПОСЛЕ вызова), БЕЗ RNG.
 * Счётчики — plain static long (допускаем гонки region-потоков: для
 * диагностики частот аппроксимация достаточна, скорость >> LongAdder).
 * Пустой/чужой CRUSSTY_LEVER_FLAG = сайт вообще не ретаргетится
 * (rust-гейт) + мост не определён (fail-closed по построению).
 *
 * ЧТО МЕРЯЕМ (EFFECT-маркеры в server-stdout.log, каждые ~2с):
 *   [crussty-plugin] GOALPROBE canUse evals=N rejects=M sampled_ns=S sample_n=K dt_ns=D
 *     → evals/sec, reject-rate (ветка D ванильной цепочки
 *       isRunning→disabledFlags→canBeReplacedForAllFlags→canUse),
 *     → сэмплированная стоимость canUse (1/1024 вызовов, наносек).
 *   [crussty-plugin] GOALPROBE creplaced evals=N rejects=M dt_ns=D
 *     → частота C-ветки (goalCanBeReplacedForAllFlags): сколько раз
 *       заблокированная цель ОТКАЗАЛА в замене (reject до canUse).
 *
 * ГИПОТЕЗА S88 (реордеринг предикатов, ванильность = тот же исход выбора):
 * ваниль-безопасный класс реордеринга ограничен pure-предикатами
 * (hasCommonElements/canBeReplacedBy/isRunning) — RNG-несущие canUse/
 * canContinueToUse переставлять нельзя (сдвиг потока RandomSource =
 * поведенческая дивергенция, тот же класс следа, что PARITY-FAIL ×3
 * AI-depth). Нога поставляет ИЗМЕРЕННЫЕ частоты исходов для
 * capture-матем потолка класса.
 */
public final class GoalProbeOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp468_s88probe");
    }

    private static final boolean ENABLED = leverEnabled();

    // Аппроксимация под гонками region-потоков — осознанно plain long
    // (частоты, не бухгалтерия; документировано в шапке).
    static long evals;
    static long rejects;
    static long sampledNs;
    static long samples;
    static long creplacedEvals;
    static long creplacedRejects;
    static volatile long lastFlushNs;
    static volatile long lastFlushNs2;
    private static final long FLUSH_NS = 2_000_000_000L; // 2с

    private GoalProbeOps() {}

    static {
        if (ENABLED) {
            System.err.println(
                "[crussty-plugin] GOALPROBE EFFECT armed (canUse site x1 @GoalSelector.tick bc166"
                + " + canBeReplacedBy site x1 @goalCanBeReplacedForAllFlags bc59 -> pass-through"
                + " counters; vanilla outcome bit-for-bit; lever cmp468_s88probe)");
        }
    }

    /**
     * Receiver-prepended static форма сайта
     * {@code invokevirtual WrappedGoal.canUse()Z} в GoalSelector.tick()V.
     * Ванильный вызов + счёт исхода. Исключения пробрасываются (ваниль).
     */
    public static boolean canUseProbe(WrappedGoal wg) {
        evals++;
        boolean sample = (evals & 1023L) == 0L;
        long t0 = sample ? System.nanoTime() : 0L;
        boolean r = wg.canUse(); // ваниль, throwable пробрасывается
        if (sample) {
            sampledNs += System.nanoTime() - t0;
            samples++;
        }
        if (!r) {
            rejects++;
        }
        maybeFlush();
        return r;
    }

    /**
     * Receiver-prepended static форма сайта
     * {@code invokevirtual WrappedGoal.canBeReplacedBy(WrappedGoal)Z} в
     * {@code GoalSelector.goalCanBeReplacedForAllFlags}. Ваниль + счёт.
     */
    public static boolean canBeReplacedProbe(WrappedGoal locked, WrappedGoal incoming) {
        creplacedEvals++;
        boolean r = locked.canBeReplacedBy(incoming); // ваниль
        if (!r) {
            creplacedRejects++;
        }
        maybeFlush2();
        return r;
    }

    private static void maybeFlush() {
        long now = System.nanoTime();
        long last = lastFlushNs;
        if (now - last < FLUSH_NS) {
            return;
        }
        lastFlushNs = now; // benign race: двойной флаш за окно допустим
        System.err.println(
            "[crussty-plugin] GOALPROBE canUse evals=" + evals
            + " rejects=" + rejects
            + " sampled_ns=" + sampledNs
            + " sample_n=" + samples
            + " dt_ns=" + (now - last));
    }

    private static void maybeFlush2() {
        long now = System.nanoTime();
        long last = lastFlushNs2;
        if (now - last < FLUSH_NS) {
            return;
        }
        lastFlushNs2 = now;
        System.err.println(
            "[crussty-plugin] GOALPROBE creplaced evals=" + creplacedEvals
            + " rejects=" + creplacedRejects
            + " dt_ns=" + (now - last));
    }
}
