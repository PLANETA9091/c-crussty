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

| метрика | run#10 | run#11 | run#15 (sweeps=1) | run#16 (paired) | run#17 (BENCH-4 fp=4, leg1) | run#18 (BENCH-4 fp=4, leg2) | цель |
|---|---|---|---|---|---|---|---|
| MSPT avg | 80.86ms | 84.47ms (разброс ~4% => A/B paired обязателен) | **49.6ms steady (tickmonitor, 8 окон)** | **74-80ms steady** | **76.98ms headline / 72.1ms [⚡]-окна** | **85.24ms headline / 81.7ms [⚡]-окна** | **<= 50ms** |
| TPS | ~13.5 | ~13.3 | **20.0 стабильных ~13 мин** | **12.5-13.5** | **12.8-14.6 steady** | **11.6-12.7 steady** | **20.0** |
| нужно срезать | | | | | **BENCH-4 база min-of-2 = run17: ~-40% CPU** | |

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

> **run#17 (S7-100, BENCH-4 fp=4, id 35156292165, sha 8a6988d)**: первый
> ФИКСТУР-ВАЛИДНЫЙ прогон канонического условия владельца — fake_players=4
> (BenchFake-0..3, детерминированные UUID, кольцо ±320), gates 1a/1b/1c PASS:
> spawnable 289 чанков; churn ACTIVE с summons=0 (polls=15, дельта 774 сущностей:
> item 163->814, ocelot 4->100, zombie 67->96, creeper 80->105, bee 2->19) —
> спавн/деспавн «как будто игроки есть» УПРАЖНЯЕТСЯ. База: 76.98ms headline /
> 72.1ms steady, TPS 12.8-14.6. GC: duty 0.70% wall (321 паузы, avg 19.6ms) —
> GC-FAMILY law сохраняется. Кросс-ран с run#16 НЕ парится (cpu_idx 9080657,
> у world3_art run-env нет) — структурное сравнение профилей только.
> Fresh recon (research/bench4-recon-2026-09-17/run17): профиль структурно
> стабилен vs run#16 (kernel:other 21.5/22.0, entities 12.7/12.5, chunk 10.4/9.8);
> spawn-лейн суммарно ~0.6% (NaturalSpawner 0.31 + createState 0.17 +
> checkDespawn 0.05) — owner-сценарий НЕ взрывает спавн-лейн; новинки top-leaves:
> ServerEntity.sendChanges 0.77% (visibility фейк-игроков), Entity.setDeltaMovement
> 1.10% (топ entity-лейна), frem+fmod ~1.13%. ЗАМЕНЯЕМЫХ СОЛО >= 3% В НОВОМ
> ПРОФИЛЕ НЕ ОБНАРУЖЕНО (PalettedContainer.get 3.62% — closed chunk lane).
> => run#18 35159240368 (leg 2, fp=4, master f3c82b3) деспатчен 22:45Z —
> bench-4 база min-of-2 paired; выбор следующего рычага — после absorb run#18.

> **run#18 (S7-102, leg 2, id 35159240368, sha 7747a8f-era master)**: SUCCESS
> 23:10:26Z; FIXTURE-VALIDITY VALID (churn дельта 816 / 9.4%, summons=0;
> alive-check стабилен). MSPT 85.24ms headline / 81.7ms окна, TPS 11.6-12.7,
> cpu_idx 6746569 (run#17: 9080657) — ДРУГОЙ runner, leg spread 10.7%.
> => **BENCH-4 BASELINE MIN-OF-2 = run#17: 76.98ms / TPS 12.8-14.6** (консервативная
> планка для всех будущих A/B; спред подтверждает runner-variance law).
> Профиль leg1/leg2 структурно стабилен (research/bench4-recon-2026-09-17/run18):
> PalettedContainer.get 3.42/3.62 (closed lane), optimiseRandomTick 1.96/2.56
> (refuted), advanceSeed 1.53/1.29 (refuted solo), spawn-лейн 0.73/0.6%;
> заменимых соло >=3% в min-of-2 профиле НЕТ.
> => run#19 35163894978 деспатчен 23:48Z = N=16 SCALING PROBE (сценарий, не
> модульный вин): метрики роста network/visibility (sendChanges 0.77% при N=4) +
> spawn-proximity (LocalMobCapCalculator) с N; absorb next tick — если lane
> >=3% replaceable при реалистичном N => следующий рычаг.

> **run#19 (S7-103/104, N=16 SCALING PROBE, id 35163894978)**: SUCCESS 00:13:11Z,
> FIXTURE-VALIDITY VALID (churn 747/8.4%, summons=0, alive-check стабилен), MSPT
> 57.01ms, TPS steady 16.9-20.3, cpu_idx 10088241 = САМЫЙ быстрый runner серии
> (абсолютный MSPT кросс-ран невалиден — variance law; вердикт по долям профиля).
> **N-SCALING VERDICT: REFUTED как источник рычага** — профиль N-инвариантен:
> network lane 1.47->0.85% (SHRINK при 4x игроков; sendChanges total
> 2180->1258 = 1.57->0.89%; outbound дешёв при discard-handler), spawn-лейн
> 0.6->0.7% (суб-линейный рост, NaturalSpawner 0.25->0.38%), единственный рост
> = GC-лейны +2.6pp (G1 4.48->5.99, barriers 4.50->5.61) — но alloc-shape
> закрыт GC-FAMILY LAW (STW duty 0.80% wall). ЗАМЕНЯЕМЫХ СОЛО >=3% ПРИ N=16 НЕТ.
> **НАБЛЮДЕНИЕ для владельца (не вердикт, 1 нога)**: на самом быстром runner
> серии текущий kernel держит ~19-20 TPS steady при N=16 — гипотеза «20 TPS
> блокирует контеншн shared-runner, а не kernel» требует min-of-2/pinned-runner
> для проверки. Оставшиеся пути к 20 TPS: (2) семейные агрегаты <3% патчей
> (owner-санкция на пересмотр гейта), (3) pinned/dedicated runner (owner-инфра),
> (4) N-scaling REFUTED — профиль N-инвариантен.

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
- **BENCH-4 FAKE-PLAYERS (task170, S7-99 = IMPLEMENTED, S7-100 = VALIDATED)**: каноническое условие владельца (спавн/деспавн как при игроках) + честная база для всех будущих A/B. STEP-0 контракт оффлайн-верифицирован (research/bench4-recon-2026-09-17: placeNewPlayer public + внутренний SGPL 4-arg; doSendPacket isConnected-safe; Connection.tick не тикается для самодельного Connection; keepalive 15s timeout закрыт публичным handleKeepAlive через EmbeddedChannel-стаб; Dec-2025 kernel: PlayerMobDistanceMap заменён на LocalMobCapCalculator.playersNearChunk — то же требование к фикстуре). BenchFakePlayersPlugin (real ServerPlayer, детерминированные UUID, N=4 кольцо, alive-check heartbeat) + harness FAKE_PLAYERS + report FIXTURE-VALIDITY gate. VALIDATION: run#17 35156292165 absorbed — FIXTURE-VALIDITY VALID (gates 1a/1b/1c PASS, churn дельта 774 при summons=0); база leg 1 = 76.98ms / TPS 12.8-14.6; run#18 35159240368 (leg 2, fp=4) in flight — min-of-2 paired база bench-4 эры
- entity_mirror infrastructure A/B (vehicle-dense, ENT-BP infra)
Каждый шаг — паритет-банкованный; сводные A/B после каждого семейства.

> **run#21 (S7-106, PAIR-LEG, id 35169715709)**: SUCCESS 01:37:36Z; FIXTURE-VALIDITY
> VALID; MSPT **76.01ms**, fp=4, cpu_idx 8914646 (band [8636000,9525000] — первая
> нога, прошедшая band-gate после 4 fast-fail отверганий: 6981619/7086411/
> 9958944/вне банды — каждый за ~30с вместо 25 мин).
> **ПЕРВАЯ ЛЕГАЛЬНАЯ ПАРА (S7-96d law, pair_hunter.py)**: world_sha MATCH
> (afb3a0b3ba78) + fp 4/4 + fixture VALID обе ноги + cpu Δ 1.86% <= 2% tol:
> **runA 35169715709 cpu 8914646 MSPT 76.01 | runB 35156292165 (run#17) cpu
> 9080657 MSPT 76.98 => SPREAD 1.3%** (кросс-ран спред был 10.7% — pairing
> схлопывает шум на порядок; runner-variance law подтверждена количественно).
> Парный профиль (lanes_vs_run17): структурно стабилен — все kernel-лейны в
> ±1.6pp (kernel:other -1.61, entities -0.76, network -0.37, chunk -0.18),
> шевелятся только GC-лейны (+1.3/+0.4pp, закрытая семья) и unpooled "other"
> (+2.06pp) — ЗАМЕНИМЫХ СОЛО >=3% НЕТ и на паре. BASELINE НА ПАРЕ = 76.01ms.
> Стоимость охоты: 4×~30s fast-fail + 1 полная нога = самодостаточный legal
> min-of-2 без owner-hardware (task171 доставлен end-to-end).

> **СТАТУС 20 TPS (S7-109)**: СОЛО-ЭРА ЗАВЕРШЕНА (6 подтверждений: run#11,
> N=4 min-of-2, N=16, легальная пара, run#22 кросс-класс, run#23 кросс-класс) —
> восемь STEP-0 киллов + профиль без заменимых соло >=3% на всех классах
> железа + N-SCALING REFUTED + парная проверка. BENCH-4 база: mid-класс пара
> run#17×run#21 = 76.01/76.98 (spread 1.3%); slow-класс 3 ноги = 85.24/83.74/
> 83.96 (spread 1.8%, CLASS-BIMODALITY: MSPT кластеризуется по типу VM —
> LCG шумит ±7% внутри класса, cpu-дельта внутри класса НЕ конвертируется в
> MSPT-дельту); межклассовая сепарация ~10% (slow 84 / mid 76.5 / fast 57).
> Вердиктный гейт паринга = пререгистрированное 2% правило (консервативно);
> class-paired = owner-facing гипотеза. ПУТИ ВПЕРЁД (все owner-gated):
> (2) семейные агрегаты <3% (санкция на пересмотр >=3% правила); (3) pinned/
> dedicated runner класса fast/mid (оценка железного рычага ~10-25% MSPT);
> (4) смена сценария. МОДУЛЬНЫХ РЫЧАГОВ >=3% НЕТ; инфра честных A/B
> самодостаточна и строга (2% вердикт, harness-cpu паринг, sequential).

> **СТАТУС 20 TPS (S7-110)**: FAMILY-AGG PREREGISTRATION — путь (2)
> КВАНТИФИЦИРОВАН как агрегатный рычаг (docs/FAMILY_AGG_PREREGISTRATION.md,
> STEP-0 бумажно, ДО кода). Разрез по НЕПЕРЕСЕКАЮЩИМСЯ лейнам парного профиля
> (sum ~96%), все числа из banked STEP-0: Tier A (только строго верифицированные
> ядра с bit-exact/median-exact паритетом: BATCH-RNG 1.6-1.9 + BRAIN 0.9-1.5) =
> 2.5-3.4%; Tier B (+ LevelTicks parity-safe слайсы reads 0.3-0.5 + queue <=0.5;
> signal lens 1.5-2.5 ТОЛЬКО при bit-exact доказательстве) = **3.3-6.9% >= 3%
> гейта** — первый GO-кандидат эры. Tier C (+ minecarts-assumption 0.6-1.3 +
> ENT-BP parked 1.5-1.8) = 5.4-10.0% (не пререгистрировано). План без пересмотра
> гейта: pack = ОДИН рычаг {F1 batch-RNG, F2 Brain, F3-safe}; билд по чану с
> паритет-банкингом (F1 -> F2 -> F3-reads -> F3-queue; signal — условно); ОДИН
> агрегатный A/B min-of-2 (baseline-нога = банк легальной пары 76.01/76.98,
> pack-ноги = hunt_leg_b машинерией); если pack < 3% — НИЧЕГО не landится,
> агрегат REFUTED, модульная повестка пуста. Исключены по паритет-риску:
> neighbor-glue skip, mid-tick reshape. Гипотеза «один рычаг = агрегат-пак»
> помечена для owner (одно слово = отмена, нулевой риск). Pair-машинария тёплая:
> формальная пара #2 leg dispatched (35179585066, fp=4, band-gated).

## калибровка профилировщика (banked, task165)

- **alloc-collapsed leaves НЕ равны new-сайтам**: `AABB.intersects` (чистая
  математика, 0 new байткодом) — топ-лист; `PalettedContainer.get` — 0 new.
  Интерпретация: alloc-профиль = давление корреляции (TLAB-refill окна),
  НЕ карты сайтов. Verifed-site метод: javap `new`-скан по телу метода.
- CPU leaf после JIT-инлайна = нижний физический фрейм (nextInt -> advanceSeed).

> **СТАТУС 20 TPS (S7-111)**: FAMILY-AGG билд стартовал — F1 BATCH-RNG реализован
> с parity-банкингом (первый инкремент по §125 пререгистрации; ничего не
> landится до агрегатного A/B). RandomTickOps.java: helper с инлайн-LCG (локальный
> seed, регистр-резидент; reject-pick = ноль dispatch/field-трафик), синхронизация
> Unsafe get/put на private `value` только на границах body-вызовов, setSeed
> bypass (gaussian-reset side-effect найден cfdump'ом и обойдён корректно).
> Бит-точный паритет: PASS (320K attempts, 8 сидов, 160K hit-интерливов вкл.
> nextGaussian). Следующий tick: Rust byte hook (classfile.rs surgery + retransform
> по area_map-образцу) + runtime self-test, затем F2 Brain-итераторы. Пара #2:
> 3 band-reject (один в 0.2% от band'а), 4-я нога in flight. INJECTS-ONLY цел.
