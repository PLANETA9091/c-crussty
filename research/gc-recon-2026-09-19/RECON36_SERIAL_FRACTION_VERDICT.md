# RECON-36 — SERIAL-FRACTION СЦЕНЫ X150K (EPOCH-2C P1, спека S7-171 §6/§3)

Дата: 2026-09-20 (+08) · TASK-362 · Скрипт: scripts/bench4_recon/recon36_serial_fraction.py (взвешенные сэмплы, кросс s7194+s7189)
Данные: run-{s7194-zeroalloc-v1, s7189-traveldiet-v2a}/world3-bench.zip → cpu-collapsed.txt (128,104 / 127,150 сэмплов; cpu_index 6.63M/6.95M — широкий банд OK)
Статус: офлайн-верификация P1, санкционирована спекой §9; диспатчей нет; истина — GOAL + CLAIMS

## 1. THREAD-SPLIT (профили свёрнуты без имён потоков → дискриминация по вход-пути стека)

| поток (вход-путь) | s7194 | s7189 | смысл |
|---|---|---|---|
| WORKER-ENTITY (дно = RegionTickOps.lambda$ensureHelpers$4) | 43.8% | 45.0% | 4 region_threads-потока, duty ~11%/поток |
| NATIVE-VM (libc thread_native_entry → Thread::call_run) | 36.2% | 35.2% | GC+JIT+VM (кросс-чек RECON-26: GC 33.4/32.5%) |
| **MAIN (дно = MinecraftServer.lambda$spin$2)** | **19.8%** | **19.6%** | **serial критический путь** |
| TPE/CHUNK-WORKER/прочие | 0.2% | 0.1% | шум |

Кросс-стабильность ±0.2п.п. по всем осям — раскладка надёжна.

## 2. ФАЗЫ MAIN (serial остаток в деталях, % сцены / % main)

| фаза | s7194 | s7189 | хвосты-пины |
|---|---|---|---|
| entity-tick (main-бакеты + оркестрация) | 15.6% (78.5% main) | 15.6% (79.5%) | updateFluidHeightAndDoFluidPushing 1549, PalettedContainer.get 1236, aiStep 918, getEntities 864+832, applyEffectsFromBlocks 824, bucketOf 765, parallelTick$3 793 |
| chunk-system (tracker + отправка) | 2.2% (10.9%) | 2.0% (10.4%) | TrackerTickOps.sweep 2273, ServerEntity.sendChanges 1483, newTrackerTick 1094, TrackedEntity.moonrise$tick 833 |
| random-tick | 0.8% | 0.7% | RNG-порядок = парити-риск |
| block-tick-scheduler | 0.6% | 0.6% | LevelTicks |
| server-tail/прочее | 0.6% | 0.6% | |
| fluid-sim НА MAIN | ~0% | ~0% | fluid уже внутри entity-бакетов (main+workers) |

**78-80% main = исполнение entity-бакетов** — это domain region_steal (main участвует
в тиках, чтобы финишировать последним и не парковаться на DONE — хедер RegionTickOps:
DONE-park 13.4% wall ДО region_steal → ≈0 после).

## 3. GATE P1 ЧЕСТНОСТИ (спека §6): **serial-fraction = 19.7% ≪ 70% → C ПРОДОЛЖАЕТСЯ**

## 4. ПЕРЕСМОТР ФОРМЫ C-РЫЧАГА (главный результат P1 — развилка)

Наивный Амдал «перенести работу main → большой буст» ЗДЕСЬ НЕ РАБОТАЕТ: 79% main —
это entity-бакеты, которые main УЖЕ делит с workers через region_steal. Слив main-доли
в workers: worker-нагрузка 43.8 → 59.4% сцены на 4 потока (14.85%/поток), критический
путь ПЕРЕВОРАЧИВАЕТСЯ на самого медленного worker'а. Потолок wall (текущий ≈ main-CPU
19.7 у.е. + барьерный хвост) при N воркерах и дисбалансе I:

| сценарий | wall нового крит.пути | буст wall-clock |
|---|---|---|
| N=4, идеальный баланс (I=1.0) | 14.9 + sync | **+33%** |
| N=4, I=1.15 | 17.1 | **+15%** (у бара) |
| N=4, I=1.3 | 19.3 | **+2% — мёртв** |
| N=8, I=1.1 | 8.2 + main-остаток | **+58%** (упирается в 4 vCPU раннера и GC 36%) |

**РАЗВИЛКА P2 решается одним дешёвым измерением**: per-thread tail/дисбаланс workers
НЕ наблюдаем из thread-слитого collapsed-профиля (нет имён потоков) — это главный
резидент P1. Прегегистр P2-pre-gate: один CI-лег с thread-separated профилировщиком
(async-profiler `-t` collapsed / per-thread timing) на банк-ноге → измерить
max(worker)/avg(worker) за тик и DONE-wait main.
- I ≤ 1.15 → P2 = слив main-бакетов в workers + offload main-остатков (tracker-sweep
  2.2% — прецедент TrackerTickOps; random/block 1.4% — парити-рискованный RNG-порядок)
  → потолок +25..33% > ДВОЙНОГО БАРА; первый профиль-обоснованный путь ≥+10% за эру.
- I ≥ 1.3 → P2 = ребаланс сначала: REGION_CHUNKS=8 (RegionTickOps, константа) → мельче,
  WORKERS (уже env-driven) ↑ при наличии ядер; балансовые леги до GREEN-геометрии.

## 5. ЧЕСТНАЯ АМЕНДА СПЕКИ S7-171 (§4/§3)

- Чистая оркестрация (bucketOf 765 + parallelTick$3 793 + lambda.accept 753 ≈ 1.8%
  сцены) — микро-зона <5%: «Rust-планировщик как замена Java-оркестрации» САМ ПО СЕБЕ
  НЕ рычаг. C-ценность = геометрия пула (N, размер региона, баланс) + offload
  main-остатков; Rust-сторона — носитель телеметрии/планирования, не замена оркестрации
  ради оркестрации. Спека §4 уточнена этим доком (не переписывается — пин здесь).
- GC/NATIVE 36% CPU — вне MSPT-критического пути (RECON-26), но на 4-vCPU раннере
  конкурирует за ядра с worker-пулом → расширение N без большего раннера может дать 0.
- CPU-профиль не видит WAIT main (барьеры) — после region_steal DONE-park≈0 (хедер
  RegionTickOps), GO-arrival/переключения = резидуал P2-pre-gate-лега.

## 6. ВЕРДИКТ

**P1 ЗАВЕРШЁН: serial-fraction 19.7% (гейт ≤70% пройден, C-эра продолжается);
главная развилка P2 = worker-дисбаланс I, решается одним thread-separated CI-легом;
потолок C: 0% (I≥1.3, N=4) .. +58% (N=8, I=1.1) — при I≤1.15 это первый путь ≥ ДВОЙНОГО
БАРА, обоснованный профилем, за всю эру RECON-25..35.**
