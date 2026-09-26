# RESEARCH-S91 — deadline-scheduler тик-задач вместо FIFO (ROUND-468, WILD)

Артефакты: `lab/s91/S91DeadlineHarness.java` (offline-харнесс, JDK21), `lab/s91/run3.txt` (прогон).
База: origin/master @ c1196321 (0 код-дельт в Java/Rust — исследования only).

## Гипотеза (скоуп S91, WILD)
«Планировщик с дедлайнами вместо FIFO-очередей тик-задач даст +20пп TPS».
Вопросы: (1) что за очереди на ванили (javap); (2) ванильность реордера
(порядок-чувствительные механики, ordering-канон MC-310372); (3) capture-матем.

## 1. Инвентаризация очередей (javap patched-kernel.jar, 1.21.x Paper/Moonrise)
| # | очередь | структура | порядок | дедлайн-поле |
|---|---|---|---|---|
| 1 | `LevelChunkTicks.tickQueue` (block/fluid scheduled ticks) | `java.util.PriorityQueue` + компаратор `ScheduledTick.DRAIN_ORDER` | **EDF уже в ванили**: (triggerTick, priority, subTickOrder) | есть (triggerTick) |
| 2 | `ServerLevel.blockEvents` | `ObjectLinkedOpenHashSet<BlockEventData>` (insertion-ordered = FIFO), `runBlockEvents()` = полный дрен каждый тик, неприоритезируемые → `blockEventsToReschedule` → хвост | FIFO | **нет** (`BlockEventData = record(pos, block, paramA, paramB)`) |
| 3 | `CollectingNeighborUpdater.stack` | `ArrayDeque` (LIFO-стек!) + `maxChainedNeighborUpdates` cap | LIFO, порядок-канон редстоуна | нет — реордер = vanillaity-нарушение |
| 4 | `BlockableEventLoop.pendingRunnables` (main-thread задачи) | `Queue` FIFO | FIFO | нет |
| 5 | `ChunkTaskPriorityQueue.queuesPerPriority` | `List<Long2ObjectLinkedOpenHashMap<List<Runnable>>>` — per-priority; чанк-система Moonrise = приоритизированный планировщик | priority, FIFO внутри приоритета | приоритет вместо дедлайна |

`TickPriority` = 7 уровней (EXTREMELY_HIGH..EXTREMELY_LOW). Кап тика:
`ServerLevel.tick()` → `LevelTicks.tick(gameTime, paperConfig.environment.maxBlockTicks/maxFluidTicks, …)`
(ваниль 65536). `runBlockEvents` — полный дрен (javap -c: цикл removeFirst до пустого множества).

## 2. Эмпирика стенда (world-bench лог, ROUND-468)
- spark tick-monitor MSPT: **avg 400.08ms** (min 343.82 / max 504.9) → бюджет тика ≈ 400ms;
- TPS-хвост first-of-window [19.5, 1.7, 1.9, 2.3, 2.6, 2.6] → tick wall ≈ 1/2.3 ≈ 435ms;
- GC: 124 pause events, Full=10 (фоновая нагрузка — очереди тут ни при чём);
- reference lane% реального хот-лейна: fluid-scan java-плоскость = 14.6% (банк) — очереди
  тик-задач на таком фоне не видны вовсе (спарк-профайлер lane отсутствует = <0.5%).

## 3. Офлайн-харнесс (числа run3.txt)
- A1: FIFO(ArrayDeque) drain+dispatch 65536 событий = **0.417ms (6.4 ns/event)** = lane 0.104% тика;
- A2: EDF-heap (как ванильный PriorityQueue) = **20.225ms (308.6 ns/event), x48.5 МЕДЛЕННЕЕ FIFO**
  — на равных дедлайнах heap только добавляет стоимость;
- A3: EDF без seqid-tiebreak = **99.997% инверсий порядка** (65534/65536) — реордер ломает
  insertion-семантику blockEvents (observers/noteblocks — порядок-канон);
- A4: потолок «выигрыша» = **−4.95пп** (EDF медленнее): на равных дедлайнах EDF≡FIFO по выходу;
- B1: vanilla-EDF heap на Парето-стриме (200 тиков, N=65536): **0.179 ms/тик management = 0.045%** бюджета;
- B2: глобальная FIFO на том же стриме = **100% wrong-tick исполнений** — FIFO семантически
  невалиден для scheduled ticks, дедлайн-порядок в ванили обязателен (и уже есть);
- C1..C4: worst-case (2×65536 событий/тик, management-only): FIFO 1.694 ms/тик = lane 0.42%;
  потолок = lane% × max-capture%(66.7% = x3 management) = **0.28пп**; даже x100 management = **0.42пп**.

## 4. Capture-матем (формат REFUTED_CENS)
lane% (0.42% worst-case management при ПОЛНОМ капе обоих очередей каждый тик) ×
max-capture% (66.7% супероптимистично, 99% — предел) = **0.28..0.42пп << 20пп**.
Структурные причины (не числа, но механика):
1) scheduled ticks — уже EDF+priority+subTickOrder (заменять FIFO нечем — FIFO там и не было);
2) blockEvents — единственный честный FIFO, но без поля дедлайна все дедлайны = «сейчас» →
   любой дедлайн-планировщик деградирует в FIFO с heap-оверхедом (x48 в харнессе);
3) neighbor updater — LIFO-стек порядка-канона: реордер = vanillaity-FAIL (закон 4);
4) чанк-система — уже priority-планировщик.

## Вердикт
REFUTED_CENS: «deadline-scheduler вместо FIFO» не существует как ванильный объект —
все latency-чувствительные очереди уже deadline/priority-упорядочены, единственный честный
FIFO (blockEvents) имеет вырожденные дедлайны (EDF≡FIFO) и потолок management-capture
0.28-0.42пп при полном капе; реордер порядка ломает ordering-канон (закон 4).
Диспатч CI не требуется: вердикт аналитический, оффлайн-харнесс воспроизводим
(`java S91DeadlineHarness`, JDK21).
