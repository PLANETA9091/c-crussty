# GOAL — 20 TPS на MineShield-3 (north star)

> Зафиксировано владельцем 2026-09-16: «твоя задача сделать 20 тпс, минимальный
> количество мспт на этом майншилд 3 сервере со всеми чанками форс лоадами и что
> бы и мобы спавнились, и деспавнились, когбудто бы игроки есть».
> Ранее: «никаких фоллбеков, всё должно быть ускорено а не просто менять конфиги».

## binding directive (правила ранга рычагов)

1. **NO FALLBACKS**: нативный путь либо работает и побеждает, либо рычаг REFUTED.
   Никаких «фоллбек-веток» в хот-лупе; переключение — на уровне гейта.
2. **NO CONFIG-WINS**: любые «+TPS» от правки конфигов/JVM-флагов — НЕ ускорение
   и не засчитываются в ledger. Только реальный код: Rust/JNI, ASM-patch,
   алгоритм/структуры данных, побеждающий с медианно-точной паритетом.
3. Один рычаг за раунд; STEP-0 kill-gate до кода; гейт >= 3.0% MSPT, CI A/B min-of-2.
4. INJECTS-ONLY в sandbox; CI-буты санкционированы.

## сценарий (scenario fidelity)

- Мир: реальный MineShield-3 (Purpur 1.21.10), все чанки force-load.
- **Требование владельца**: мобы спавнятся И деспавнятся, «как будто игроки есть».
  Сейчас natural spawning не активен (0 игроков в бенче) — сущности мира живы
  (Brain.tick 6.85% подтверждает), но цикл спавна не нагружает тик.
  => BENCH-SCENARIO upgrade (task166+): fake-player注入 для per-player spawn
  алгоритма (без сети, серверные ServerPlayer-заглушки). Это сценарий бенча,
  НЕ игровая правка; фиксируется как требование к world-bench-4.

## база и цель

| метрика | run#10 | run#11 | run#15 (sweeps=1) | run#16 (paired) | цель |
|---|---|---|---|---|---|
| MSPT avg | 80.86ms | 84.47ms (разброс ~4% => A/B paired обязателен) | **49.6ms steady (tickmonitor, 8 окон)** | **74-80ms steady** | **<= 50ms** |
| TPS | ~13.5 | ~13.3 | **20.0 стабильных ~13 мин** | **12.5-13.5** | **20.0** |
| нужно срезать | | | | **базовая линия после run#16: ~-40% CPU** | |

> **RUNNER-VARIANCE LAW (run#16, S7-96d)**: 20 TPS run#15 REFUTED min-of-2 —
> идентичный снапшот (item_frame 2714(160)), идентичные входы, но 12.5-13.5 TPS /
> 74-80ms => shared-runner CPU variance доминирует кросс-рановые базовые линии
> (±30-35% MSPT). Все будущие A/B — только same-boot или paired по
> (world_sha256, runner_cpu_index) — run-env.txt пишется в каждый прогон.
> North star 20 TPS остаётся ОТКРЫТ; базовая линия = 12.5-13.5 TPS / 74-80ms.

> **run#15 (S7-96c, дубль-агент)**: первый прогон канонического условия владельца
> (summon_sweeps=1: 75 summons, F4 churn ACTIVE — дельта 629 сущностей, drowned
> 11->47, item 148->772) — **20.0 TPS / ~49.6ms MSPT steady**. НО: никаких модульных
> оптимизаций в этом прогоне не шипилось (только observability), а run#14 на том же
> мире/forceload/модуле дал 13-14 TPS / ~68-75ms. Ведущая гипотеза дельты:
> **вариативность живого снапшота мира** (world URL = живой экспорт сервера
> владельца: равновесие item/mob-накопления меняется между скачиваниями; run#15
> boot: item 148 -> 772 за прогон = свежий снапшот, накопление в реальном времени).
> => north star НЕ объявляется достигнутым до min-of-2: run#16 (те же входы)
> деспатчен 19:53Z. Если run#16 ~50ms — снапшот-вариативность подтверждена как
> ДОМИНИРУЮЩИЙ фактор базовой линии, и все будущие A/B обязаны качать мир один раз
> на пару прогонов (paired download discipline) либо использовать pinned-снапшот.

## MSPT budget ledger (run#11, % от CPU тика, обновляется каждый раунд)

| лейн / кластер | presence | replaceable-ядро (верифицировано) | статус |
|---|---|---|---|
| entity/mobs всего | 43.3% | — | главный резерв |
| Brain.tick кластер | 6.85% | STEP-0 сделан (S7-98): replaceable ядро = iterators 0.47-0.80 + getNode 0.16-0.18 + views 0.08-0.20 + половина SELF ~0.15-0.25 = **0.9-1.5%**; itable-диспатч 0.45-0.48 и canStart-тела (PalettedContainer/PathTypeCache/Long2Object) НЕЗАМЕНЯЕМЫ | **REFUTED (task168) — 0.9-1.5% << 3%** |
| GC (STW + barriers) | 9.5% | **CLOSED (S7-97 GC-FAMILY LAW)**: 9.5% = concurrent worker CPU (G1CM/RebuildRemSet), НЕ MSPT; STW duty 0.50-0.56% wall (gc.log run#12/15: eden 2.4GB, 412-598 MB/s, young-GC каждые 4-6s); relief = alloc_share x 0.5% => BlockPos 2-3% alloc => <=0.015% MSPT; Brain-LHM 0.4-0.69% => <=0.005% | **REFUTED (task166) — alloc-shape семейство закрыто** |
| random-tick lane | 5.46% | advanceSeed 1.57% + BlockPos-alloc (hit-only, 2-3% alloc => <=0.015% MSPT) + хвост SELF; batch-RNG REFUTED соло | REFUTED соло (task165); BlockPos-слайс закрыт GC-law (task166) |
| scheduled-tick drain (LevelTicks.tick) | 8.28/11.55% | STEP-0 сделан (S7-97): tickBlock контракт верифицирован; decompose: reads 2.65/3.77 + signal 1.18/1.43 + queue 0.5-0.76 + glue 0.29/0.62 + mid-tick 1.57/1.83 + tail — ВСЕ <3% соло | REFUTED-solo (task167); family-bank parked |
| minecarts | **2.15/2.58%** (measure run#12/15; старые ~5.3% = bucket mirage) | STEP-0 сделан (S7-98): весь лейн под гейтом — move 0.55/0.73 + applyEffectsFromBlocks 0.40/0.46 + hopper 0.18/0.21 + push/pickup 0.12/0.17 + fluid-push 0.12/0.14; perfect-lens bound < 3% | **REFUTED (task169) — whole-lane < gate** |
| Villager | 3.47% | пересекается с Brain | через Brain |
| chunk lane | 9.8% | per-get lens REFUTED; batch-lens REFUTED (3.3% потолок) | closed |
| worldgen | 0.0% | — | closed (измерено) |
| boats (owned) | 0.76% | < 4% гейта | REFUTED (task164) |
| ENT-BP соло | 1.5-1.8% | < 3% | parked (инфра для vehicle A/B) |

## арифметика честного пути к 20 TPS

Сумма всех ЗАКРЫТЫХ честных соло-рычагов сегодня ~5-7% MSPT — мало.
Путь к 40%: только **агрегатные семейства** (каждый патч проходит свой гейт):
- ~~GC-SHAPE~~ **CLOSED (S7-97)**: GC-family law — alloc-shape relief = alloc_share x STW-duty 0.5%; верхний потолок всего семейства < 0.5% MSPT
- ~~REDSTONE-LENS~~ **REFUTED-solo (S7-97)**: drain-lane 8.28/11.55% decompose => все слайсы <3%; family-bank parked
- ~~BRAIN-LENS~~ **REFUTED (S7-98)**: replaceable ядро 0.9-1.5% (dispatch + canStart-тела незаменимы)
- ~~minecarts~~ **REFUTED (S7-98)**: весь лейн 2.15/2.58% < гейта (mirage ~5.3% исправлен)
- **BENCH-4 FAKE-PLAYERS (task170, S7-99 = IMPLEMENTED)**: каноническое условие владельца (спавн/деспавн как при игроках) + честная база для всех будущих A/B. STEP-0 контракт оффлайн-верифицирован (research/bench4-recon-2026-09-17: placeNewPlayer public + внутренний SGPL 4-arg; doSendPacket isConnected-safe; Connection.tick не тикается для самодельного Connection; keepalive 15s timeout закрыт публичным handleKeepAlive через EmbeddedChannel-стаб; Dec-2025 kernel: PlayerMobDistanceMap заменён на LocalMobCapCalculator.playersNearChunk — то же требование к фикстуре). BenchFakePlayersPlugin (real ServerPlayer, детерминированные UUID, N=4 кольцо, alive-check heartbeat) + harness FAKE_PLAYERS + report FIXTURE-VALIDITY gate. VALIDATION RUN 35156292165 dispatched — absorb next tick
- entity_mirror infrastructure A/B (vehicle-dense, ENT-BP infra)
Каждый шаг — паритет-банкованный; сводные A/B после каждого семейства.

> **СТАТУС 20 TPS (S7-99)**: СОЛО-ЭРА ЗАВЕРШЕНА — восемь STEP-0 киллов подряд
> (ENT-BP, BOAT, BATCH-RNG, GC-SHAPE-1, REDSTONE, BRAIN-LENS, MINECARTS);
> верифицированных соло-рычагов >=3% НЕ ОСТАЛОСЬ. РАУНД S7-99: bench-4
> fake-players РЕАЛИЗОВАН (task170) — validation run 35156292165 dispatched;
> после absorb — fresh recon НОВОГО профиля (spawn-лейн + выросший AI-лейн),
> из которого выбирается следующий рычаг (если появится >=3% replaceable-ядро).
> Остальные пути: (2) семейные агрегаты <3% патчей (требуют пересмотра правила
> гейта владельцем); (3) инфраструктура: pinned/dedicated runner для честных A/B.
> НЕДОСТИЖИМО из текущего профиля без смены сценария — смена сценария = bench-4.

## калибровка профилировщика (banked, task165)

- **alloc-collapsed leaves НЕ равны new-сайтам**: `AABB.intersects` (чистая
  математика, 0 new байткодом) — топ-лист; `PalettedContainer.get` — 0 new.
  Интерпретация: alloc-профиль = давление корреляции (TLAB-refill окна),
  НЕ карты сайтов. Verifed-site метод: javap `new`-скан по телу метода.
- CPU leaf после JIT-инлайна = нижний физический фрейм (nextInt -> advanceSeed).
