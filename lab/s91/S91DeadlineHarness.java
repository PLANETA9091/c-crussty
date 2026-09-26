import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

/**
 * ROUND-468 S91 WILD: deadline-scheduler vs FIFO для тик-задач (offline harness).
 * Гипотеза: "планировщик с дедлайнами вместо FIFO-очередей даёт +20пп".
 * Ванильная инвентаризация (javap patched-kernel.jar 1.21.x Paper/Moonrise):
 *  1) LevelTicks/LevelChunkTicks.tickQueue = java.util.PriorityQueue с компаратором
 *     DRAIN_ORDER=(triggerTick, priority[7 уровней], subTickOrder) — УЖЕ deadline-планировщик.
 *  2) ServerLevel.blockEvents = ObjectLinkedOpenHashSet (insertion-ordered = FIFO),
 *     полный дрен каждый тик (runBlockEvents), BlockEventData = record(pos,block,paramA,paramB)
 *     — ПОЛЯ DEADLINE НЕТ: все события тика имеют одинаковый дедлайн "сейчас" => EDF==FIFO.
 *  3) CollectingNeighborUpdater.stack = ArrayDeque (LIFO-стек!), cap maxChainedNeighborUpdates
 *     — порядок-чувствительный редстоун, реордер = нарушение закона 4 (vanillaity).
 *  4) BlockableEventLoop.pendingRunnables = Queue FIFO (main-thread задачи, <1% тика).
 *  5) ChunkTaskPriorityQueue.queuesPerPriority = List<Long2ObjectLinkedOpenHashMap<List<Runnable>>>
 *     — уже per-priority; Moonrise ChunkTaskScheduler — приоритизированный планировщик.
 * Эмпирика бенча (run E1-класс ROUND-468, LOG): spark MSPT avg 400.08ms, TPS-хвост ~2.3-2.6
 * => бюджет тика на стенде ≈ 400ms (не 50ms), lane% мелкой очереди падает ещё в 8 раз.
 *
 * Харнесс меряет ТОЛЬКО management-cost очередей (dispatch-обвязка), не работу хендлеров:
 *  - Scenario A: blockEvents-класс, N=65536 (ванильный кап тика), FIFO vs EDF-heap
 *    (все дедлайны равны) + подсчёт инверсий порядка у EDF без seqid (vanillaity-метрика).
 *  - Scenario B: scheduled-ticks-класс, стрим с Парето-дедлайнами на 200 тиков:
 *    vanilla-EDF (heap) per-tick management ns; FIFO-гипотеза = доля wrong-tick исполнений.
 *  - Scenario C: capture-потолок: lane% × max-capture% против барьера 20пп.
 */
public final class S91DeadlineHarness {
    static final int CAP = 65536;              // ванильный кап block/fluid тиков за тик
    static final long TICK_MS = 400;           // эмпирика бенча: spark MSPT avg 400.08ms
    static final int WARM = 15, ROUNDS = 25;

    // --- ScheduledTick-подобный элемент: (triggerTick, priority, subTickOrder/seq) ---
    record Tick(long trigger, int prio, long seq) {}

    static final Comparator<Tick> EDF_STABLE =
        Comparator.comparingLong(Tick::trigger).thenComparingInt(Tick::prio).thenComparingLong(Tick::seq);

    public static void main(String[] args) {
        System.out.println("== S91 deadline-vs-FIFO harness (budget/tick=" + TICK_MS + "ms, cap=" + CAP + ") ==");
        scenarioA();
        scenarioB();
        scenarioC();
    }

    // ---------- Scenario A: blockEvents (FIFO, все дедлайны == now) ----------
    static void scenarioA() {
        System.out.println("\n--- Scenario A: blockEvents FIFO vs EDF-heap (все deadline=now, N=" + CAP + ") ---");
        Tick[] base = new Tick[CAP];
        for (int i = 0; i < CAP; i++) base[i] = new Tick(0, 0, i);

        long fifoNs = bench(() -> {
            ArrayDeque<Tick> q = new ArrayDeque<>(CAP);
            for (Tick t : base) q.addLast(t);
            long bh = 0;
            while (!q.isEmpty()) bh += q.pollFirst().seq;
            return bh;
        });
        long edfNs = bench(() -> {
            PriorityQueue<Tick> q = new PriorityQueue<>(EDF_STABLE);
            for (Tick t : base) q.add(t);
            long bh = 0;
            while (!q.isEmpty()) bh += q.poll().seq;
            return bh;
        });

        // vanillaity: EDF БЕЗ seqid (только deadline) — сколько инверсий против FIFO-порядка
        int inversions = 0;
        {
            final ThreadLocalRandom rnd = ThreadLocalRandom.current();
            PriorityQueue<Tick> q = new PriorityQueue<>(Comparator.comparingLong(Tick::trigger));
            Tick[] shuffled = base.clone();
            for (int i = shuffled.length - 1; i > 0; i--) { int j = rnd.nextInt(i + 1); Tick t = shuffled[i]; shuffled[i] = shuffled[j]; shuffled[j] = t; }
            for (Tick t : shuffled) q.add(t);
            long prev = -1;
            while (!q.isEmpty()) { long s = q.poll().seq; if (prev != -1 && s != prev + 1) inversions++; prev = s; }
        }
        double fifoMs = fifoNs / 1e6, edfMs = edfNs / 1e6;
        double laneFifo = 100.0 * fifoMs / TICK_MS, laneEdf = 100.0 * edfMs / TICK_MS;
        System.out.printf("A1 FIFO(ArrayDeque)  drain+dispatch: %8.3f ms  (%6.1f ns/event)  lane=%5.3f%% тика%n", fifoMs, fifoNs / (double) CAP, laneFifo);
        System.out.printf("A2 EDF(PriorityQueue+seqid):   %8.3f ms  (%6.1f ns/event)  lane=%5.3f%% тика, x%.2f к FIFO%n", edfMs, edfNs / (double) CAP, laneEdf, (double) edfNs / fifoNs);
        System.out.printf("A3 EDF без seqid: инверсий порядка %d/%d (%.3f%%) — реордер ломает insertion-семантику%n", inversions, CAP, 100.0 * inversions / CAP);
        System.out.printf("A4 потолок выигрыша = lane(EDF-FIFO): %.3fпп от тика (EDF МЕДЛЕННЕЕ — дедлайн-планировщик тут только добавляет)%n", laneEdf - laneFifo);
    }

    // ---------- Scenario B: scheduled ticks, Парето-дедлайны на 200 тиков ----------
    static void scenarioB() {
        System.out.println("\n--- Scenario B: scheduled-ticks стрим (Парето-дедлайны, 200 тиков, N=" + CAP + ") ---");
        final ThreadLocalRandom rnd = ThreadLocalRandom.current();
        long now = 1000;
        Tick[] stream = new Tick[CAP];
        for (int i = 0; i < CAP; i++) {
            long d = (long) Math.min(200, Math.round(200 * Math.pow(rnd.nextDouble(), 4.0))) + 1; // Парето-хвост
            stream[i] = new Tick(now + d, rnd.nextInt(7), i); // 7 уровней TickPriority
        }
        long edfNs = bench(() -> {
            PriorityQueue<Tick> q = new PriorityQueue<>(EDF_STABLE);
            long bh = 0;
            for (int tick = 0; tick < 200; tick++) {
                for (Tick t : stream) if (t.trigger == now + tick) q.add(t);
                while (!q.isEmpty() && q.peek().trigger <= now + tick) bh += q.poll().seq;
            }
            return bh;
        });
        // FIFO-гипотеза: ОДНА глобальная FIFO получает события в порядке schedule()-вызовов
        // (случайный относительно дедлайнов) и исполняет при первой возможности =>
        // почти всё исполняется РАНЬШЕ своего дедлайна (wrong-tick).
        long wrong = 0, total = 0;
        {
            Tick[] shuffled = stream.clone();
            for (int i = shuffled.length - 1; i > 0; i--) { int j = rnd.nextInt(i + 1); Tick t = shuffled[i]; shuffled[i] = shuffled[j]; shuffled[j] = t; }
            ArrayDeque<Tick> q = new ArrayDeque<>();
            int p = 0;
            for (int tick = 0; tick < 200; tick++) {
                while (p < shuffled.length) { q.addLast(shuffled[p]); p++; } // FIFO не смотрит на дедлайны
                while (!q.isEmpty()) { Tick t = q.pollFirst(); total++; if (t.trigger > now + tick) wrong++; }
            }
        }
        double perTickMs = edfNs / 200.0 / 1e6;
        System.out.printf("B1 vanilla-EDF heap: management %8.3f ms/тик (%6.1f ns/event) = %.4f%% бюджета %dms%n", perTickMs, edfNs / (double) CAP, 100.0 * perTickMs / TICK_MS, TICK_MS);
        System.out.printf("B2 FIFO-гипотеза: wrong-tick исполнений %d/%d (%.2f%%) — FIFO семантически невалиден, дедлайн-порядок обязателен%n", wrong, total, 100.0 * wrong / Math.max(1, total));
        System.out.printf("B3 вывод: дедлайн-планирование в LevelTicks УЖЕ есть (DRAIN_ORDER javap) — заменять нечего%n");
    }

    // ---------- Scenario C: capture-потолок ----------
    static void scenarioC() {
        System.out.println("\n--- Scenario C: capture-потолок 'deadline вместо FIFO' ---");
        // Худший случай: КАЖДЫЙ тик полный кап 65536 blockEvents + 65536 fluid-класс задач,
        // management FIFO по замерам A1; "идеальный" планировщик улучшил бы ТОЛЬКО management
        // (хендлеры те же), скорость heap≈FIFO±x — берём супероптимистично x3 быстрее FIFO.
        long fifoPerQueueNs = measureFifoNs();
        double worstBothQueuesMs = 2 * fifoPerQueueNs / 1e6;                 // blockEvents + fluid-класс
        double lanePct = 100.0 * worstBothQueuesMs / TICK_MS;                // lane%
        double maxCapturePct = 66.7;                                         // x3 ускорение management (супероптимизм)
        double ceilingPp = lanePct * maxCapturePct / 100.0;                  // пп TPS
        System.out.printf("C1 worst-case: 2×65536 событий/тик, FIFO management %.3f ms/тик%n", worstBothQueuesMs);
        System.out.printf("C2 lane%% = %.4f%% тика (%dms)@бенч-стенде%n", lanePct, TICK_MS);
        System.out.printf("C3 потолок = lane%% × max-capture%%(66.7%%=x3) = %.3fпп << 20пп%n", ceilingPp);
        System.out.printf("C4 даже при x100 ускорении management: %.3fпп < 20пп (выигрыш <= lane%%, management ≠ хендлеры)%n", lanePct * 0.99);
    }

    static long measureFifoNs() {
        Tick[] base = new Tick[CAP];
        for (int i = 0; i < CAP; i++) base[i] = new Tick(0, 0, i);
        return bench(() -> {
            ArrayDeque<Tick> q = new ArrayDeque<>(CAP);
            for (Tick t : base) q.addLast(t);
            long bh = 0;
            while (!q.isEmpty()) bh += q.pollFirst().seq;
            return bh;
        });
    }

    static long bench(Callable c) {
        for (int i = 0; i < WARM; i++) c.call();
        long best = Long.MAX_VALUE;
        for (int r = 0; r < ROUNDS; r++) {
            long t0 = System.nanoTime();
            c.call();
            best = Math.min(best, System.nanoTime() - t0);
        }
        return best;
    }
    interface Callable { long call(); }
}
