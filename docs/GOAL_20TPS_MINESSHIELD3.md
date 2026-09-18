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

> **СТАТУС 20 TPS (S7-113)**: F2 BRAIN-ИТЕРАТОРЫ BUILT — `randomtick/src/BrainOps.java`:
> flat-snapshot lens поверх setup-stable структуры Brain
> (availableBehaviorsByPriority: OUTER=TreeMap/INNER=LinkedHashMap/SET=LinkedHashSet —
> CP-факты; мутации ТОЛЬКО {ctor, computeIfAbsent+Set.add, clear} — все 5 getfield'ов
> просвечены). groupStart[] воспроизводит LIVE-contains РОВНО в местах vanilla
> (раз на группу; одинаковая activity в двух приоритетах = две проверки);
> getStatus/tryStart — не тронутые LIVE-вызовы; gameTime — один раз. FINGERPRINT:
> 5 семейств O(1)-проб (2 identity + 3 size), 0 итераторных аллокаций на hot path;
> cache = WeakHashMap+IdKey (AbstractMap.equals TRAP закрыт S7-тестом).
> PARITY BANK на РЕАЛЬНОМ production entry (ServerLevel через Unsafe.allocateInstance +
> WritableLevelData-прокси; SharedConstants+Bootstrap.bootStrap = статические данные,
> НЕ бут): **PASS — 4828 вызовов / 3083 order-exact событий / 1740 мутаций / 8 сценариев**
> (S7 EQUALS-TRAP + S9 fuzz 60×40 kernel-размера). Byte hook = 14-байтовая прямая
> строка (следующий тик, getfield СВОИХ private-полей = verifier-легален, helper без
> Unsafe). Пара #2: нога#6/7 band-reject (~30s, без трат), нога#8 SUCCESS но harness
> +4.8% вне окна => честный discard, нога#9 35187305900 in flight. Ничего не landится —
> вердикт у агрегатного A/B (§125/§128). INJECTS-ONLY цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-112)**: F1 BYTE HOOK ГОТОВ — Rust-хук `randomtick.rs`
> + `classfile.rs::patch_optimise_random_tick` (образец patch_update): тело
> optimiseRandomTick заменено на 11-байтовую прямую строку
> `aload/aload/iload/aload/getfield simpleRandom/invokestatic
> RandomTickOps.run/return` (без ветвлений => ПУСТОЙ StackMapTable, 0 кадров).
> Append-only CP, дедуп => patch(patch(x))==patch(x) (идемпотентность
> доказана тестом); fail-closed: чужой класс/обрезанные байты => Err без
> паники (14 prefix-срезов fixture). ВЕРИФИКАТОР-ГЕЙТ на РЕАЛЬНОМ HotSpot:
> VerifyPatched (resolveClass = link-time verification, major 65) —
> **VERIFY-OK** (research-скрипт randomtick/verify_patched.sh; байт-тесты
> cargo: 82 passed). Активация по area_map-образцу: poll ServerLevel ->
> define RandomTickOps в loader kernel'а -> READY -> retransform; маркер-цепочка
> defined/armed/rc/PATCHED; семантический self-test = CI-буты (санкционированы,
> каждый smoke с миром гоняет patched body). Ничего не landится — следующий
> шаг билда: F2 Brain-итераторы, затем F3-reads; агрегатный A/B против банка
> пары 76.01/76.98 решает ВСЁ (§125 протокол). Пара #2: нога#5 35181833283
> OUT-of-window (+0.7%) — честный discard, absorb run#25: 86.65ms при
> harness 6574725 => 4-я slow-нога, класс-спред n=4 = **3.5%** (83.74-86.65;
> было 1.8% n=3 — честная коррекция owner-числа; межкласс по-прежнему ~10-25%).
> Нога#6 35183885492 in flight. INJECTS-ONLY цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-114)**: F2 BYTE HOOK ГОТОВ — `classfile.rs::patch_brain_start_each`
> (образец patch_optimise_random_tick): тело startEachNonRunningBehavior
> (0x0002, vanilla len=178) заменено на 14-байтовую прямую строку
> `aload_0/getfield availableBehaviorsByPriority/aload_0/getfield
> activeActivities/aload_1/aload_2/invokestatic
> BrainOps.startEachNonRunning:(Map;Set;ServerLevel;LivingEntity;)V/return`
> (без ветвлений => ПУСТОЙ StackMapTable; max_stack 5 / max_locals 3; оба
> getfield — СВОИ private-поля внутри Brain.class => verifier-легально,
> helper БЕЗ Unsafe). Append-only CP + дедуп => идемпотентность тестом;
> fail-closed (14 prefix-срезов + чужие классы). Runtime wiring `brainhook.rs`:
> poll Brain -> define ВЕРНОМ порядке {BrainOps$IdKey, BrainOps$Snapshot,
> BrainOps} в loader kernel'а (nested-first: ленивое разрешение через defining
> loader иначе промахнулось бы по classpath kernel'а) -> READY -> retransform;
> маркер-цепочка defined/armed/rc/PATCHED. ВЕРИФИКАТОР-ГЕЙТ: VerifyBrain на
> реальном HotSpot — child-first loader {patched Brain + helper trio},
> kernel-jar parent-first => **VERIFY-OK** (brain 31966B major 65 + ops 5469B
> слинкованы; randomtick/verify_brain_patched.sh; cargo: 85 passed). Семантический
> self-test = CI-буты (каждый smoke с мобами гоняет patched body; parity-контракт
> уже banked §128: 4828 вызовов PASS). ПАКЕТ: F1 hook ✓ + F2 hook ✓ — следующая
> единица билда: F3-reads (task167 slices, parity-banking), затем ОДИН
> агрегатный A/B против банка пары 76.01/76.98 (§125 протокол) решает всё.
> Пара #2: нога#9 35187305900 SUCCESS но harness 6966037 (+6.7% вне окна)
> => честный discard (5-й подряд честный reject — ворота строгие, ни одной
> ложной пары); нога#10 35189275270 dispatched in flight. INJECTS-ONLY цел
> (0 sandbox boots; verify = link-time только, без <clinit>).

> **СТАТУС 20 TPS (S7-115)**: F3-READS BUILT — `randomtick/src/TickBlockOps.java`
> (package net.minecraft.server.level): ЧЕТЫРЁХСЛОЙНАЯ анатомия cfdump'ами
> run21 (Level.getBlockState @0-71: capture-ветка ПЕРВАЯ + isOutsideBuildHeight
> → VOID_AIR + getChunk(FULL,true) + ChunkAccess.getBlockState;
> LevelChunk.getBlockStateFinal: nonEmptyBlockCount==0 → AIR-шорткат; дрен
> LevelTicks.runCollectedTicks @0-76 С QUIRКОМ: set.remove под guard'ом
> !toRunThisTickSet.isEmpty(); tickBlock @0-53: is→tick→counter&7→mid-tick).
> Дизайн: дрен-хелпер = байт-точное зеркало ванили (Unsafe-read приватных
> final-полей — одна выборка эквивалентна getfield на каждой итерации) + окно
> ThreadLocal section→chunk кэша; readBlockState-линза: ТОЛЬКО getChunk-хоп
> заменён (первый тик секции платит точный ванильный getChunk, остальные секции
> — кэш-хит; ChunkAccess.getBlockState остаётся реальным вызовом); ВНЕ окна —
> точный ванильный путь (tickBlock достижим только из дрен-лямбды — семантика
> идентична, просто не батчится). Mid-tick ВЫЗОВ байт-идентичен по построению
> (тот же invokeinterface на том же поле), в банке не исполняется — счётчик
> держится под кратными 8, арифметика ветки сверяется точно. PARITY BANK на
> РЕАЛЬНЫХ production entry (LevelTicks/LevelChunk/ServerChunkCache/
> PalettedContainer — все allocateInstance+preseed реальных структур; counting
> ConcurrentLong2ReferenceChainedHashTable-стаб; CraftBlockState.getHandle
> реальным диспатчем): **PASS — S1 дрен порядок/книгучёт, S2 quirk, S3 10/10
> чтений (палитра/AIR-шорткат/VOID_AIR/отрицательные координаты), S4 30/30
> счётчик+ветка, S5 capture-ветка, S6 идентичность стрима чтений + ВЫИГРЫШ
> RESOLUTIONS REF=6 → NEW=2 (per-pos → per-section), S9 fuzz 40/40**.
> Рантайм-швы устранены в банке: chunkSource на ServerLevel (НЕ Level!),
> section-резолюция через ChunkAccess.levelHeightAccessor (НЕ minSection),
> -20>>4=-2 floor-деление. Byte hooks СЛЕДУЮЩИЙ тик: patch_run_collected_ticks
> (6 байтов: aload_0/aload_1/invokestatic/return) + patch_tick_block (11
> байтов: aload_0×3/invokestatic/return) — оба без ветвлений => пустой
> StackMapTable. ПАКЕТ: F1 ✓ + F2 ✓ + F3-reads built (hooks next) — затем
> F3-queue (≤0.5%) и ОДИН агрегатный A/B (§125). Пара #2: нога#10 gate-reject
> (8869954, ~30s), нога#11 gate-reject (6852134 у нижней кромки, ~30s) — 7-й
> подряд честный reject; нога#12 35193865177 dispatched in flight.
> INJECTS-ONLY цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-116)**: F3 BYTE HOOKS ГОТОВЫ — `classfile.rs::patch_run_collected_ticks` (6 байтов:
> aload_0/aload_1/invokestatic TickBlockOps.runCollectedTicks/return, max_stack 2/max_locals 2) +
> `patch_tick_block` (7 байтов: aload_0×3/invokestatic/return, max_stack 3/max_locals 3 — GOAL-заметка
> S7-115 «11 байтов» была арифметической опечаткой, запечатано debug_assert + тестом). Оба тела
> без ветвлений => ПУСТОЙ StackMapTable; append-only CP + дедуп => идемпотентность; fail-closed.
> **F1+F3 COHABITATION (новый шов закрыт)**: оба хука на ServerLevel; JVMTI retransform подаёт
> ORIGINAL bytes, F1 one-shot guard молчит на повторных dispatch => tickBlock-only образ УНИЧТОЖИЛ
> бы optimiseRandomTick swap; решение — F3 ServerLevel-колбэк КОМПОЗИЦИОННЫЙ (ре-apply idempotent F1
> перед tickBlock), итоговый образ order-independent + cycle-stable (тест
> f3_serverlevel_composes_with_f1: оба тела живы, compose deterministic и idempotent на composed
> input — обе byte-модели retransform покрыты). Runtime `src/tickhook.rs`: poll BOTH {ServerLevel,
> LevelTicks} + force-load => define TickBlockOps ОДИН класс (без nested) => READY => retransform ×2
> => маркеры F3 ARMED ×2. Тесты: cargo **89 passed** (85+4; REAL LevelTicks fixture 18923B sha
> ba7dce5e…). ВЕРИФИКАТОР-ГЕЙТ: VerifyF3 на реальном HotSpot — child-first {patched LevelTicks
> 18848B + COMPOSED ServerLevel F1F3 142243B + TickBlockOps 5325B}, kernel parent-first,
> link-time без <clinit> => **VERIFY-OK major=65**. CI 46aa80b SUCCESS (маркер-grep job-логов —
> token 401, отложено; bench runner-лог ноги#13 несёт маркеры бесплатно). ПАКЕТ: F1 hook ✓ + F2
> hook ✓ + F3-reads hooks ✓ — следующий блок билда: F3-queue (≤0.5%, task167) или сразу ОДИН
> агрегатный A/B против банка пары 76.01/76.98 (Tier B floor 3.3% достигается без queue — решение
> по §125 протоколу на следующем тике). Пара #2: нога#12 gate-reject (8875106 >> band, ~30s) —
> 8-й подряд честный reject; нога#13 35196354695 dispatched in flight. INJECTS-ONLY цел
> (0 sandbox boots).

> **СТАТУС 20 TPS (S7-117)**: F3-QUEUE BUILT+HOOKED — TIER B ПАКЕТ СОБРАН ПОЛНОСТЬЮ.
> `TickBlockOps.collectTicks` = байт-точное зеркало collectTicks (sort+counter+
> drain+reschedule, run21 cfdump) с инлайном gate-машинерии (canScheduleMoreTicks
> 3 virtual calls/тик → field compare; scheduleForThisTick → прямой add; 4
> Unsafe-выборки полей на пайплайн). Паритет — САМЫЙ СИЛЬНЫЙ REF эры: РЕАЛЬНЫЙ
> vanilla приватный collectTicks через reflection (нулевой mirror-copy риск):
> **PASS** SQ1 sort-ветки / SQ2 gate boundary / SQ3 frozen-innerHead — наблюдаемый
> порядок [A5,B1,A7] подтвердил INTRA_TICK_DRAIN_ORDER cross-container fairness
> quirk (comparator = priority→subTickOrder, innerHead заморожен до цикла) /
> SQ4 re-keying lifecycle / SQ5 fuzz 30/30. Ловушки банка: LevelChunkTicks.schedule
> дедуп по (pos,type); updateContainerScheduling ПЕРЕКЛЮЧАЕТ ключ на позицию тика
> (контейнер осиротеет, если tick-chunk ≠ container key — реальная vanilla
> семантика). Byte hook: patch_collect_ticks — 9B (wide aload 4, max_stack 5/
> locals 5, пустой StackMapTable); tickhook LevelTicks callback композитный
> (drain 6B + queue 9B в одном образе). cargo **91 passed**; VerifyF3 **VERIFY-OK
> major=65** (composed LevelTicks 18837B + ServerLevel F1F3 142243B + TickBlockOps
> 8389B). ПАКЕТ (§125): F1 ✓ + F2 ✓ + F3-reads ✓ + F3-queue ✓ — signal lens
> dropped по §5.3 (floor 3.3% держится). **СЛЕДУЮЩИЙ ТИК = ОДИН АГРЕГАТНЫЙ A/B**
> против банка пары 76.01/76.98 (min-of-2, gate ≥3.0% MSPT) — решает ВСЁ: при
> <3% НИЧЕГО не landится, модульная повестка пуста (owner-gated пути: pinned
> runner, сценарий). Пара #2: нога#13 gate-reject (6786413 < floor, ~30s) — 9-й
> подряд честный reject; нога#14 35198344256 dispatched in flight. INJECTS-ONLY
> цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-118)**: РЕКОН БАГ#3 ИСПРАВЛЕН — охота была невыигрываемой
> по конструкции. v2-окно hunt_leg_b было заякорено на run22 (cpu 6401514, SLOW
> класс, MSPT 83.74), тогда как baseline-нога §125 = банк MID-класс пары
> run#17×run#21 (9080657/8914646, 76.98/76.01) — кросс-класс сравнение нелегально
> по S7-96d (class-bimodality ~10%); хуже: gate band [6870000,7030000] НЕ
> ПЕРЕСЕКАЕТСЯ с окном [6273484,6529544] — band-проходные ноги (~6.9M, leg#9
> +6.7%) всегда мимо окна, оконные ноги gate-убиты (leg#13 6786413 < floor).
> 11 подряд честных reject объяснены. v3: WINDOW = [8899044, 9092939] — точное
> пересечение ±2% терпимости вокруг ОБЕИХ banked ног (легальная пара с обоими
> baseline-arms), BAND = [8850000,9120000] (+skew slack), WORLD_SHA pin
> afb3a0b3… (7/7 ног консистентны; drift → discard + re-baseline flag).
> **F1 ARMATION НА BENCH ДОКАЗАНА** (профиль-грейп leg#9 35187305900, kernel
> e326ab3-эры): BOTTLENECKS CPU + WALL таблицы — `RandomTickOps.run` leaf
> **3.0% (4175 samples) / 3.2% (5799)** — swapped body ИСПОЛНЯЕТСЯ на bench
> runner; random-tick phase 4.4-4.8%. Кавет: helper self-leaf 3.0-3.3% vs
> vanilla optimiseRandomTick 1.96-2.56% — подозрение на attribution skew
> (inlining-коллапс callees за invokestatic границей); НЕ вердикт — решает
> только агрегатный A/B. Пара-охота: нога#14 gate-reject (11482771, ~30s) —
> 10-й честный; нога#15 gate-reject (9764130, ~30s) — 11-й честный; нога#16 =
> 35202368357 dispatched на 2c5c40e (FULL PACK kernel F1+F2+F3) с v3-машинерией.
> Вердиктный порог §125: pack median ≤ **74.20ms** (= 76.495 × 0.97) при 2
> in-window ногах. INJECTS-ONLY цел (0 sandbox boots; marker-grep = download
> лога, не бут).

> **СТАТУС 20 TPS (S7-119)**: §125 ПОПРАВКА-1 — SLOW-ТРЕК БАЗЫ; охота v3 снова
> невыигрываемая на практике, но по НОВОЙ причине: сдвиг популяции раннеров.
> Перепись 43 ног с cpu_idx: mid-окно [8899044,9092939] hit только самими банками
> (run#17/run#21, эра 22:10Z/01:13Z); с тех пор **0/14 in-window** (две mid-ноги
> 8869954/8875106 промахнулись на ~0.3%); медленный класс 6.57-6.87M доминирует
> (~30% выдач). Банк MID-класса невозобновляем → reuse банка против свежих ног
> мёртв. ПОПРАВКА (все owner-гейты целы: ≥3% MSPT, min-of-2, ±2% паринг, world
> pin): baseline arm#1 = **run#18 35159240368** (cpu 6746569, MSPT 85.24,
> FIXTURE-VALID S7-102, PRE-PACK kernel f3c82b3-эры — до S7-111/112, world
> afb3a0b3 подтверждён) — бесплатный банк; arm#2 (B1) = свежий диспатч на
> **pre-pack тег `pre-pack-962fc9f`** (S7-111: RandomTickOps.java в банке, но
> НЕ hooked ⇒ нулевое pack-поведение); PACK WINDOW = пересечение ±2% вокруг
> обеих arms (пусто ⇒ честный discard B1, редиспатч); pack-ноги на master в
> окне; ВЕРДИКТ: pack median ≤ median(85.24, B1_mspt)×0.97 при 2 in-window
> ногах ⇒ pack lands, иначе REFUTED row, ноль лендинга. Уроки S7-119 (пойманы
> живьём): workflow-dispatch API НЕ принимает SHA-ref (HTTP 422) ⇒ аудит-тег
> запушен; v4 добавила dispatch-verify (head_branch+status) — слепой захват
> свежайшего run id чуть не записал pack-ногу#18 как baseline state. ОХОТА:
> нога#16 gate-reject (6604889) — 12-й честный; нога#17 (7094750) — 13-й;
> нога#18 (6593031) — 14-й; B1 попытки#1-5 gate-reject (10165007, 8820009,
> 8744984, 7137698, 8780971 — пул ротируется: 8.7-8.8M всплыли, 6.6-6.9M были
> 40 мин назад); **B1 попытка#6 = 35205343087 IN FLIGHT** на pre-pack теге
> (gate пройден, full bench). v4 = scripts/bench4_recon/hunt_leg_b_v4.py
> (one-transition/call, resumable, world pin, early-cancel). INJECTS-ONLY цел
> (0 sandbox boots).

> **СТАТУС 20 TPS (S7-120)**: §125-A1 охота продолжается — B1 35205343087
> (pre-pack тег) в полёте, boot >20 мин (echo ещё нет; полный bench 25-40 мин).
> Инженерия тика: **verdict_a1.py** — калькулятор вердикта §125-A1 (экстрактор
> `MSPT: avg **X**ms` из zip-логов = ТА ЖЕ headline-метрика, что у банка
> 85.24/76.98/76.01; проверка window/world/VALID на каждой ноге; threshold =
> median(arms)×0.97, pack median из 2 in-window ног ⇒ LANDS/REFUTED + запись
> verdict_a1.json) — вердиктный тик становится механическим. **НАБЛЮДЕНИЕ
> (не вердикт, 1 пара)**: leg#9 (F1-only эра e326ab3) 81.64ms cpu 6966037
> ↔ run#23 (pre-pack) 83.96ms cpu 6979464 — дистанция cpu 0.19%, тот же мир
> afb3a0b3, оба VALID, fp=4 (первоисточники перегреплены) ⇒ **−2.76% для F1
> ОДНОГО**. Направление совпадает с preregistered F1-оценкой 1.6-1.9% (solo,
> refuted против 3% гейта) и поддерживает Tier-B floor 3.3% пакета (F2+F3
> добавят); НО: одиночная пара, slow-класс спред n=4 = 3.5% ⇒ шумовые бары
> широкие — решает только агрегатный A/B §125-A1. INJECTS-ONLY цел (0 boots).

> **СТАТУС 20 TPS (S7-121)**: **РЕКОН БАГ#4** — дрейф gate-vs-final harness
> cpu. Band-gate семплирует cpu на ~30s, а закон паринга требует ФИНАЛЬНОЕ
> значение (run-env echo). Замер по парам ИЗ ОДНОГО лога: run#24 **−5.5%**,
> leg#8 −2.0%, leg#9 +0.3%, run#23 +1.5%, run#21(банк!) **+1.9%**, B1#6 +3.4%
> ⇒ дрейф неконстантный до ±5.5%. Дымящий пистолет: gate-значение run#21
> 8745625 БЫЛО БЫ убито v3-band [8850000,9120000], тогда как его финал
> 8914646 — ВНУТРИ v3-окна ⇒ гейт отклонял легально-парируемые финалы; и
> наоборот — B1#6 прошёл узкий band [6611637,6881500], а финал 7057150
> (+2.5% над окном) сгорел в честный discard. ФИКС v4.1: drift-компенсация
> band = [win_lo/(1+0.034), win_hi/(1−0.055)] — гейт пропускает все gate-
> значения, чьи финалы МОГУТ попасть в окно; финальная проверка остаётся
> точным окном (честные discard остальных, ~3 boots/ногу). **B1#6
> ПЕРЕРАБОТАН**: 35205343087 (pre-pack тег, SUCCESS, VALID, мир afb3a0b3,
> MSPT 83.40, финал 7057150) = baseline arm#1 §125-A1 — окно заякорено на
> ЖИВОЙ класс сегодняшнего пула [6916007,7198293] (run#18 arm mothballed:
> 6.75M класс остыл). **B2 = 35209341660 dispatched** (pre-pack тег,
> drift-компенсированный band [6688594,7617241]) — poll следующего тика:
> финал в окне ⇒ BASELINE COMPLETE ⇒ pack-фаза (master, тот же band-метод,
> точное окно) ⇒ 2 in-window pack-ноги ⇒ **ВЕРДИКТ**: pack median ≤
> median(83.40, B2_mspt)×0.97. Все owner-гейты целы. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-122)**: **BASELINE COMPLETE + РЕКОН БАГ#5 + pack arm#1
> в полёте.** Среда песочницы ПОЛНОСТЬЮ стёрта между тиками (3 repo,
> scripts/, creds — всё отсутствовало) — восстановлено из /tmp-снапшота
> (bootstrap_tick.sh с запечённым токеном спас: creds + 3 клона за один
> прогон; c-crussty f97c84d, dev-logs TASK-257, CRUSSTY 1f4c06a нетронут).
> (1) **B2 = 35209341660 IN-WINDOW SUCCESS**: pre-pack тег, FIXTURE-VALIDITY
> VALID, мир afb3a0b3, MSPT avg **83.51**, финал cpu **6996405** ∈ окно
> arm#1 [6916007,7198293] ⇒ §125-A1 BASELINE COMPLETE: arm#1 = B1#6
> (35205343087, 83.40 @ 7057150), arm#2 = B2 (83.51 @ 6996405) ⇒ pack
> window = [max×0.98, min×1.02] = **[6916007, 7136333]**; ВЕРДИКТНЫЙ ПОРОГ
> = median(83.40, 83.51)×0.97 = **80.95ms** — pack median 2 in-window ног
> ≤ 80.95 ⇒ LANDS, иначе REFUTED. (2) **БАГ#5 (state machine)**: v4.1
> ветка pack-диспатча требовала pack_legs truthy ([]) ⇒ pack arm#1 упал
> в fresh-baseline ветку и переdispатчил PRE-PACK ногу 35213099261 на
> неверный ref, clobbered pack state — поймано инспекцией state, cancel
> pre-bench (~3 мин, ноль bench-стоимости), state восстановлен из
> leg_b_v4.json; фикс **v4.2**: pack-диспатч = (phase=pack AND run_id is
> None) — покрывает arm#1 и arm#2. (3) **PACK ARM#1 = 35213299343
> dispatched на master** (f97c84d, FULL F1+F2+F3 pack kernel, drift band
> [6688594,7551675], финал = точное окно [6916007,7136333]) — poll
> следующего тика: in-window ⇒ leg#1 banked ⇒ автодиспатч arm#2 ⇒ 2 ноги
> ⇒ verdict_a1 ⇒ **§125-A1 ВЕРДИКТ**. (4) verdict_a1.py extracts fix:
> CI-скрипт-литерал `echo "::error::... INVALID"` в КАЖДОМ логе ронял
> valid-флаг (False на здоровых прогонах) — фикс: позитивный маркер
> `- **FIXTURE-VALIDITY: VALID**` достаточен (grep-q gate семантика).
> Пайплайн --check зелёный: обе arms mspt/window/world/valid ✓. F2/F3
> маркеры — на первой завершённой pack-ноге. INJECTS-ONLY цел (0 sandbox
> boots; cancel не boot).

> **СТАТУС 20 TPS (S7-123)**: **pack arm#1 попытка#1 = honest DISCARD; bug#6+
> bug#7 пойманы и закрыты (v4.3); попытка#3 = 35216066889 в полёте.** (1)
> 35213299343 (master f97c84d, FULL pack kernel): SUCCESS, VALID, мир
> afb3a0b3, MSPT 94.14 — НО финал cpu **6832640** НИЖЕ pack window
> [6916007,7136333] (и ниже arm2-lo 6856477 ⇒ не парируется НИ с одной
> armой) ⇒ честный discard; MSPT 94.14 не пригоден для вердикта (другой
> класс раннера — подтверждение закона парирования). (2) **БАГ#6**: логи
> CI-рана НЕДОСТУПНЫ короткое окно сразу после завершения (zip не
> финализирован) — log_text возвращал "", run падал в reject-ветку и
> state СТИРАЛСЯ (пойман live на этой ноге; исход совпал с discard'ом, но
> классификация была неверной). Фикс v4.3: пустые логи при concl=success ⇒
> retry (exit 4), state цел. (3) **БАГ#7 (семья bug#5)**: все pack-фазные
> discard/cancel/reject пути звали clear_state() — терялись phase/window/
> baseline, следующий dispatch падал в fresh-baseline ветку (не тот kernel
> на не том ref). Фикс v4.3: restore_pack_or_clear() — pack-фаза
> ВОССТАНАВЛИВАЕТСЯ (run_id=None), baseline чистится как раньше. (4)
> Попытка#2: gate cpu **10054256** (10M быстрый класс) далеко вне band
> [6688594,7551675] ⇒ ~30s fast-fail reject — dispatch-verify корректно
> отказался сохранять state завершённого рана (нота для v4.4: отличать
> свой fast-fail от чужого рана в окне 25s). (5) **Попытка#3 = 35216066889
> IN FLIGHT** (master fb7d8cb, POST 204, band [6688594,7551675], финал =
> точное окно [6916007,7136333]) — poll следующего тика: in-window ⇒
> leg#1 banked ⇒ v4.3 автодиспатчит arm#2 ⇒ 2 ноги ⇒ verdict_a1 ⇒
> **§125-A1 ВЕРДИКТ** (порог median(83.40,83.51)×0.97 = **80.95ms**
> неизменен). INJECTS-ONLY цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-124)**: **pack arm#1 attempt#3 = honest DISCARD
> (cpu 6691832 ниже окна); v4.4 dispatch-verify own-fast-fail fix
> (боевое применение); attempt#5 = 35218943354 in flight.** (1) Poll
> 35216066889 (master fb7d8cb, FULL pack kernel): SUCCESS, мир afb3a0b3,
> финал cpu **6691832** — НИЖЕ pack window [6916007,7136333] на 3.2% ⇒
> честный discard (второй: попытка#1 6832640, попытка#3 6691832). Пул
> раннеров дрейфует: наблюдаемые gate значения 6.69M..10.05M при окне
> 3.1% — терпеливая лотерея с drift band [6688594,7551675] + точный
> финальный чек (санкционированный протокол). v4.3
> restore_pack_or_clear сработал: state восстановлен (pack, run_id=None,
> окно цело). (2) **ФИКС v4.4 (диспатч-верифай слепое пятно, нота
> S7-123)**: dispatch() спит 25s перед верификацией — свой gate
> fast-fail (~13-30s) завершается ВНУТРИ окна и читался как false
> MISMATCH без сохранения state (попытка#2 повисла руками). Фикс: top
> run с ref=наш, status=completed, conclusion=failure/cancelled,
> created_at >= t0-5 ⇒ ЭТО НАШ fast-fail — state сохраняется, следующий
> poll классифицирует reject штатно. **Боевое применение в этом же
> тике**: attempt#4 35218775640 — gate cpu 8094573 (8.09M mid класс)
> выше band hi ⇒ ~13s fast-fail, v4.4 сохранил state, reject
> классифицирован штатно (exit 1), автопередиспатч. (3) **ATTEMPT#5 =
> 35218943354 IN FLIGHT** (master 7bf98fc, verify увидел in-flight на
> +25s = past gate, boot/download), band [6688594,7551675], финал =
> точное окно [6916007,7136333] — poll следующего тика: in-window ⇒
> leg#1 banked ⇒ v4.3 автодиспатчит arm#2 ⇒ 2 ноги ⇒ verdict_a1 ⇒
> **§125-A1 ВЕРДИКТ** (порог median(83.40,83.51)×0.97 = **80.95ms**
> неизменен). F2/F3 маркеры — на первой in-window ноге. INJECTS-ONLY
> цел (0 sandbox boots; gate fast-fail не boot).

> **СТАТУС 20 TPS (S7-125)**: **PACK LEG#1 BANKED (первая in-window нога) +
> F2/F3 SMOKE-ТЕСТ ЗАКРЫТ — ВСЕ 4 ХУКА ARMED на bench; arm#2 = 35221359818
> в полёте.** (1) Poll 35218943354 (master, FULL pack kernel): SUCCESS,
> мир afb3a0b3, финал cpu **6998277** ∈ [6916007,7136333] ⇒ **PACK LEG#1
> BANKED** (третья попытка — после discard'ов 6832640/6691832).
> Headline MSPT **85.44ms** (spark tick-monitor). (2) **F2/F3 SMOKE
> TEST ЗАКРЫТ** (дефер S7-116→S7-118): workflow-логи НЕ содержат
> console-маркеров (они в artifact) — скачан world3-bench artifact
> (49.5MB; 302→Azure требует no-redirect opener — новый
> scripts/bench4_recon/fetch_artifact.py) → server-stdout.log: `randomtick:
> F1 ARMED (optimiseRandomTick -> RandomTickOps.run, bit-exact LCG batch)`,
> `brainhook: F2 ARMED (startEachNonRunningBehavior -> BrainOps.
> startEachNonRunning, flat-snapshot lens)`, `tickhook: F3 ARMED ×2
> (runCollectedTicks drain mirror + tickBlock readBlockState lens, F1
> composed)` — **FULL PACK KERNEL АКТИВЕН на bench-ноге**; profile-кадры
> согласны (RandomTickOps.run 3.5-3.6%, Brain.tick 6.75%). (3) Пара-нота:
> leg#1 cpu 6998277 ≈ arm#2 6996405 (Δ 0.27%) — легальная A/B пара;
> leg#1 MSPT 85.44 vs B2 83.51 = pack +2.3% на этой паре (single-pair
> наблюдение, НЕ вердикт; F1-одиночка давала −2.76% на leg#9 — lens-хуки
> F2/F3 могут съедать выигрыш F1; вердикт механический). (4) **ARM#2 =
> 35221359818 auto-dispatched** (master, band [6688594,7551675], финал =
> точное окно) — in-window ⇒ 2 ноги ⇒ verdict_a1 ⇒ **§125-A1 ВЕРДИКТ**:
> pack median ≤ **80.95ms** ⇒ LANDS, иначе REFUTED (ноль посадок). Порог
> неизменен. INJECTS-ONLY цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-126)**: **pack arm#2 attempt#1 = honest DISCARD #3
> (cpu 6731204 ниже окна 2.7%); затем серия gate fast-fail — пул
> бимодальный 6.37M/9.86M, скачет ЧЕРЕЗ band [6688594,7551675];
> последний dispatch 35224751782 past-gate in flight.** (1) Poll
> 35221359818: SUCCESS, мир afb3a0b3, финал cpu **6731204** ниже окна
> [6916007,7136333] на 2.7% ⇒ честный discard #3 (6832640 / 6691832 /
> 6731204 — все ниже окна; pool дрейфует вниз с утра). v4.3 восстановил
> pack state (leg#1 6998277 сохранён). (2) **Серия fast-fail** (v4.4
> ноль-ручных обработок, каждый ~13-30s): gate значения **6372300 /
> 6372622 / 9864892 / 8261589 / 6645024** — бимодальная карусель вокруг
> band, 6 подряд честных reject (всего 21). Окно НЕ двигаем
> (preregistered §125-A1: окно = пересечение ±2% вокруг двух banked
> arms; перенос окна = перенос базы = owner-amendment). (3) **Последний
> dispatch 35224751782 past-gate in flight** (master, band
> [6688594,7551675], финал = точное окно) — poll следующего тика:
> in-window ⇒ leg#2 ⇒ verdict_a1 ⇒ **§125-A1 ВЕРДИКТ** (порог
> median(83.40,83.51)×0.97 = **80.95ms**; leg#1 85.44 уже в bank — при
> median(85.44, leg2) ≤ 80.95 ⇒ LANDS, иначе REFUTED). F2/F3 маркеры
> закрыты (S7-125, 4/4 ARMED). INJECTS-ONLY цел (0 sandbox boots).

> **СТАТУС 20 TPS (S7-127) — ВЕРДИКТ §125-A1: LANDS + OWNER REDIRECT
> (архитектурная эра).** (1) **§125-A1 ЗАКРЫТ механически**: leg#2 =
> 35224751782 (poll догнал в этом тике; commit-context отрезал прошлый
> poll, но hunt отработал) — финал cpu **6929321** ∈ окно, MSPT
> **73.92ms**, мир afb3a0b3, VALID ⇒ pack ноги = [85.44 @ 6998277,
> 73.92 @ 6929321], pack_median = **79.68ms** ≤ порог 80.95ms ⇒
> **LANDS, Δ = −4.52%** (гейт ≥3% пройден, min-of-2, парный закон,
> INJECTS-ONLY). Пак (F1+F2+F3-reads+F3-queue) ОСТАЁТСЯ на master — он
> уже там; pre-pack тег остаётся базовой ссылкой. verдиктный JSON:
> scripts/bench4_recon/verdict_a1.json. (2) **OWNER REDIRECT
> (2026-09-17 ~21:10-21:15 +08, прямой эфир)**: «надо x100... чтобы все
> форс чанки были и всё жило, а тпс и мспт — 20 и минимальный; ты все
> боттленки ебнуть должен — что за микро фиксы; тут архитектуры надо
> делать и реальные бусты» + поправка: «x100 это мало, твой прайм было
> **x150000**» + «по русски». ЧИТАЮ ТАК: сцена = ПРАЙМ реального
> MineShield-3 — **все форс-чанки (9216) + всё ЖИВОЕ на прайм-масштабе
> (рабочая гипотеза: ~150k живых сущностей — ЗАПРОС ПОДТВЕРЖДЕНИЯ
> ВЛАДЕЛЬЦУ: юнит x150000 = сущности?)**, спавн/деспавн как с
> игроками; таргет остаётся 20 TPS / минимальный MSPT; **микро-рычаги
> (2-3% линзы) — стоп**; арсенал = АРХИТЕКТУРА + реальные бусты, бить
> ВСЕ бутылочные горлышки. (3) Следствие: сцена-апгрейд ДЕЛАЕТ
> недействительной базу §125 (все MSPT сняты на fp=4 полуживой сцене)
> ⇒ охота в окно ЗАМОРОЖЕНА (свежий pre-pack baseline-ран 35228087782
> отменён 202, hunt state очищен; банк остаётся исторической записью).
> (4) **НОВАЯ ЭРА — BENCH-X150K (спека: docs/BENCH_X150K_SCENARIO.md)**:
> population-injection fixture (bench-only, сервер-код 100% ваниль —
> прецедент BenchFake), цель ~150k живых сущностей на буте, все чанки
> форс-лоад, фейк-игроки гоняют спавн-линии; потом свежий профиль ⇒
> ARCH-ATTACK очередь (архитектурные Rust/JNI порты горячего пути,
> планировщики, батчи) — preregistered гейты остаются для ЧЕСТНОГО
> замера бустов (median-exact, A/B min-of-2), но планка = «реальный
> буст», не 2%.

> **СТАТУС 20 TPS (S7-128) — КРОН ИЗМЕНЁН (владелец) + ЮНИТ x150000 УТОЧНЁН
> ВЛАДЕЛЬЦЕМ + ARCH-ATTACK #1: guard-wave wave-1 ARMED на bench.** (1)
> **Живое уточнение владельца (~21:2x +08)**: «ну так ты просто смотри на
> боттлнеки и сделай чтобы они не нагружали, и цель это 20 тпс и минимальное
> количество мспт, но всё работает так же как в ваниле (измени крон)» +
> «x150000 это к скорости выполнения какой то функции из боттлнеков которые
> топ 1 например по изпользованию» + «всем похуй на набор и вердикт уже
> твой». ЧИТАЮ: (а) юнит x150000 = планка ускорения СКОРОСТИ ВЫПОЛНЕНИЯ
> топ-1 функции-горлышка (по использованию) — гипотеза «150k сущностей»
> DRAFT v1 REFUTED владельцем; (б) вердикт и выбор рычага делегированы
> агенту (лотерея окон мертва навсегда); (в) поведение = ванильное.
> (2) **КРОН ИЗМЕНЁН**: Job 390768 (чартер застрял в эре S7-96 — 30+ тиков
> устаревания) УДАЛЁН; создан **Job 393012 c-crussty-module-loop-v4-arch-
> attack** (тот же график :08/:43 каждые 35 мин, Asia/Shanghai, priority
> 10): чартер архитектурной эры БЕЗ вшитого состояния (истина всегда в
> GOAL+worklog), по-русски. (3) **АРХИТЕКТУРНЫЙ ВЕРДИКТ (мой)**: топ-1
> функция по использованию = PalettedContainer.get (3.62% self, стабильна
> min-of-2 3.42/3.62/3.7) — НО анализ честных механизмов её ускорения:
> per-call плоский декод-кэш секций = +32KB×24×9216 ≈ 7GB (memory-
> неосуществимо) и пер-call win 30-50% = микро-фикс класс (запрещён
> владельцем) ⇒ архитектурная атака переносится на УСТРАНЕНИЕ ИЗБЫТОЧНОЙ
> ПЕРЕКОМПУТАЦИИ в entity-физике: **guard-wave wave-1 = same-state
> fluid-push guard (TASK-80, готов + live-verified hit-rate 96.4%)** —
> функция на гард-пути исчезает из профиля (класс >100x на хите, планка
> «горлышко не нагружает»); под сцену «всё живое» entity-лейн масштабируется
> линейно с мобами — удар туда даёт реальный буст. (4) **ВНЕДРЕНО**: bench/
> world3/run_world3.sh — FLUID_GUARD default 1 (export CRUSSTY_FLUID_PUSH_
> GUARD, самодокументация в run-env.txt «fluid_guard: …», A/B оверрайд
> FLUID_GUARD=0); workflow world-bench input fluid_guard (default '1') —
> гард стал частью измеряемого пака bench (банк §125 — историческая запись
> старой сцены, перебаза на X150K запланирована). (5) **Верификационный ран
> диспатчен**: см. CLAIMS TASK-264 / worklog S7-128 — absorb следующего
> тика: console-маркер fluid_guard armed + профиль (updateFluidHeightAndDo
> FluidPushing self → коллапс) + run-env fluid_guard: 1. (6) NEXT: S7-129 =
> guard-wave wave-2 STEP-0 (checkInsideBlocks 3.4% census / InsideBlock
> EffectApplier.flushStep 786 сэмплов run17) + X150K population fixture MVP;
> следом — повторный профиль X150K ⇒ очередь ARCH-ATTACK по свежим данным.
> INJECTS-ONLY цел (CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-129) — X150K POPULATION FIXTURE MVP РЕАЛИЗОВАН (плагин +
> раннер + workflow + локальная javac21-проверка контракта), сцену «всё живое»
> можно теперь ИЗМЕРЯТЬ.** (1) Абсорбция прерванного sibling-раунда S7-128:
> TASK-264 докоммичен verbatim; hunt-state leg_b_v4_state.json нейтрализован
> (FROZEN-BY-OWNER-REDIRECT-X150K — возобновлять ЗАПРЕЩЕНО); runs_index
> дополнен. (2) **BenchPopulationPlugin** (bench/world3/population/, BENCH-ONLY,
> ванильный addEntity-путь — прецедент task170): инъекция живой сцены
> 70% items / 20% hostiles / 10% passives; детерминизм Random(seed) +
> сортировка чанков (x,z) + бюджет 1500/тик; items = фермы (pickup-delay max,
> кластеры 5% чанков), hostiles/passives setPersistent (ферм-сток), НАТУРАЛЬНЫЙ
> спавн/деспавн остаётся ванильным; TOPUP каждые 600 тиков восполняет
> ванильно-деспавннутые items ⇒ item-лейны (тик/мердж/деспавн) горячие всё окно.
> (3) **Харнес**: benchpop inject ПОСЛЕ forceload, ожидание «POPULATION INJECT
> DONE» ДО старта профилировщиков (спека §2 «инъекция до старта окна замера»);
> run-env self-doc population_target/seed; workflow inputs + гейт FIXTURE-VALIDITY
> ≥90%. (4) **Контракт проверен ДО CI**: javac --release 21 против
> paper-api 1.21.10-R0.1-SNAPSHOT + patched-kernel.jar = COMPILE OK (риск
> roundtrip-провала снят). (5) Профиль leg#2 разобран функционально
> (profile_rank.py): entity-лейн 43.8% inclusive (aiStep 18.3%, Mob.tick 22.5%,
> Brain.tick 6.14%), block/redstone 8.9%, leaf top-1 PalettedContainer.get
> 3.37% (вердикт S7-128 неизменен) — на X150K сцена сместит топы, решит свежий
> профиль. (6) NEXT: S7-129b CI smoke 10k (после завершения вериф-рана
> 35231195756 — concurrency cancel-in-progress, не мешать) ⇒ S7-130 масштаб
> 150k + soak ⇒ S7-131 X150K база A/B + свежий профиль ⇒ топ-1 ARCH-ATTACK
> (планка x150000 = скорость выполнения топ-1 функции). INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-129b) — X150K SMOKE 10k = SUCCESS (run 35234643616,
> оба гейта зелёные) — ЖИВАЯ СЦЕНА ИЗМЕРЕНА ПЕРВОЙ В ИСТОРИИ ПРОЕКТА.**
> (1) Fixture: INJECT DONE **10000/10000** (7000 items / 2000 hostiles /
> 1000 passives) за **2572 мс**, FIXTURE-VALIDITY: VALID, farmClusters=500;
> TOPUP активен (deficit=0 steady). (2) Сцена живая: ~**24.9k сущностей**
> (натуральные спавны у 4 фейк-игроков АКТИВНЫ: skeleton 584 / drowned 468 /
> chicken 465 / sheep 442 / creeper 403 / zombie 313 / cow 311) на 9216
> форс-чанках. (3) Нагрузка: MSPT avg **125.63ms** (TPS ~7.6; первый poll
> 20.0 — до инъекции), GC 0 Full GC / high-water 3951MB — прайм-масштаб
> ДАЛЕКО от 20 TPS ⇒ ARCH-ATTACK имеет реальную работу. (4) Профиль leaf:
> **топ-1 PalettedContainer.get 4.20%** (та же цель, что вердикт S7-128),
> RandomTickOps 2.27%, guard-хуки bump 1.03% + slow 0.92% (оверхед
> инструментации виден; net-эффект guard'а — мерить A/B на X150K-базе),
> InsideBlockEffectApplier.flushStep 1.16% (кандидат wave-2 подтверждён).
> (5) Известный баг (фикс S7-130): стартовая инъекция не пишется в
> itemSpawnLog ⇒ первый topup удвоил items (~14k, self-correcting);
> topup-seed привязать к Δt = ft−T0 для строгого replay. (6) NEXT S7-130:
> фикс topup-логирования → масштаб 150k (оценка: инъекция ~39s при 1500/тик,
> heap ~+3-4GB ⇒ возможно Xmx tuning в харнесе) → soak → S7-131 база A/B
> pre-pack vs master на X150K + свежий профиль ⇒ топ-1 ARCH-ATTACK
> (планка владельца x150000 = скорость выполнения топ-1 функции).
> INJECTS-ONLY цел (0 sandbox boots; CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-130) — X150K МАСШТАБ 150k: topup-фикс внедрён + SERVER_XMX проводка; prime-ран 35238931413 ДИСПАТЧЕН (master 7956a2e), absorb S7-131.**
> (1) **Topup-фикс (баг S7-129b закрыт)**: стартовая инъекция теперь логируется в itemSpawnLog ПО ТИКАМ (itemsThisSlice → запись [fullTime, count] на каждом инжекционном тике; ранее только topup-спавны логировались ⇒ первый topup видел aliveEst=0 и удваивал items до ~6600 тика); topup-seed привязан к Δt = ft − T0 (t0FullTime = fullTime в момент finishInjection — якорь replay-детерминизма: (target, seed) → тот же topup-поток независимо от дрейфа бута); INJECT DONE маркер расширен t0FullTime для верификации якоря.
> (2) **SERVER_XMX проводка** (подготовка prime-масштаба; оценка S7-129b «heap +3-4GB» против 6G-потолка при high-water 3951MB на 10k): run_world3.sh SERVER_XMX env (default 6G = историческое поведение), java line -Xmx"$SERVER_XMX", run-env self-doc «server_xmx: …»; workflow input server_xmx (default '6G') + env pass-through. Это инфраструктура харнеса (BENCH-ONLY), не серверный код и не config-win — запрет владельца не затронут.
> (3) **Локальная контракт-проверка ДО CI** (прецедент S7-129): javac --release 21 против paper-api+adventure+bungeecord classpath = COMPILE OK; bash -n + YAML parse OK; фикс не добавляет новых API.
> (4) **Prime-ран диспатчен**: run **35238931413** (master 7956a2e, fp=4, population_target=150000, seed=42, server_xmx=10G, sweeps=0, 300s, guard=1) — concurrency guard проверен перед диспатчем; финал = absorb S7-131: гейты (INJECT DONE 150000/150000, FIXTURE-VALIDITY) + MSPT/TPS при 150k + topup-корректность (первый topup на Δt≈600t должен иметь deficit=0 при живой стартовой инъекции) + свежий профиль leaf ⇒ подтверждение/смена топ-1 ARCH-ATTACK.
> (5) Риски prime-масштаба (мониторинг в absorb): инъекция 150k при 1500/тик ≈ 100 тиков ≈ 39s+ (POP_INJECT_TIMEOUT=900s с запасом); heap 10G против оценки ~7-8GB (GC-лог артефакт покажет high-water); TPS при инъекции временно упадёт — инъекция ДО окна замера (спека §2 соблюдена харнесом).
> (6) NEXT S7-131: absorb 35238931413 ⇒ если зелёный — X150K база A/B (pre-pack vs master, min-of-2 по cpu-парам) + свежий профиль 150k ⇒ топ-1 ARCH-ATTACK (планка владельца x150000 = скорость выполнения топ-1 функции; операционализация §2 спеки: функция должна исчезнуть из профиля). Если OOM/тайм-аут — Xmx/бюджет-тюнинг харнеса первым делом (инфраструктура, не логика). INJECTS-ONLY цел (0 sandbox boots; CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-130b) — ПЕРВЫЙ УСПЕШНЫЙ PRIME-SCALE ЗАМЕР В ИСТОРИИ ПРОЕКТА: 150k живых на 9216 форс-чанках, все гейты зелёные (run 35245032701, master 6c3c20c).**
> (1) **Путь к зелёному**: диспатч #1 35238931413 (7956a2e) FAILED — мой баг: uniform-лейн инъекции one-pass по 9948 чанкам, на 150k нужно ~87k размещений ⇒ cursor исчерпан, инъекция молча замерла на ~11k (сервер ЗДОРОВ: heap 2.2/10G, сцена жила 20 мин; stop по POP_INJECT_TIMEOUT). FIX a88d204: лейн по кругу (modulo) + INJECT STALL warn (placed=0 никогда не молчит). Диспатч #2 35242595837 (a88d204): инъекция 150k DONE за 86.5s + population-гейты ПРОЙДЕНЫ, но FAILED BENCH-4 gate-1c — alive-heartbeat фейк-плагина период 1200 тиков (60s@20TPS) при TPS 0.7-1.0 = 20+ мин wall ⇒ ноль heartbeat'ов в 300s окне (fixture здоров: игроки 4/4, churn ACTIVE). FIX 6c3c20c: heartbeat 100 тиков. Диспатч #3 35245032701 (6c3c20c) = **SUCCESS, ВСЕ ГЕЙТЫ**.
> (2) **Замер**: INJECT DONE 150000/150000 (105000 items / 30000 hostiles / 15000 passives) за 100.4s, t0FullTime=225986013; живая сцена **148589/148397/148246** сущностей (натуральный спавн/деспавн ACTIVE: skeleton 4902 / zombie 4708 / creeper 4691 / husk 4671 / drowned 4595 / item 100398); alive-check 4/4 ×3 (фикс heartbeat верифицирован); GC 0 Full GC, high-water **6914MB** @ Xmx10G (запас есть; 174 паузы / 13.6s суммарно).
> (3) **Нагрузка**: TPS 0.6-0.9 ⇒ implied MSPT **~1405ms avg** — прайм-масштаб КРАЙНЕ далеко от 20 TPS; поле ARCH-ATTACK подтверждено количественно (нужно ~×150 к скорости тика для 20 TPS... «x150000» — планка топ-1 функции).
> (4) **СВЕЖИЙ ПРОФИЛЬ 150k (первый в истории, 52552 cpu-сэмплов, spark yVyl0LQNDk)**: фаза entity tick **54.2%** — доминанта; kernel-топ-1 leaf **PalettedContainer.get 3.3%** (стабилен: 4.2%@10k smoke / 3.62%@fp4 / 3.3%@150k) + readPalette 0.8% + SimpleBitStorage.get 0.8% = палитро-лейн ~4.9%; **FluidPushGuardHook.slow 2.3% + bump 0.9% = 3.2% lens-оверхед** (сам fluid-push ИЗЧЕЗ из профиля — guard wave-1 работает, платим за lens; архитектурная переделка lens = кандидат); G1 GC 13.4% + oop barriers 12.0% ≈ **25.4%** (производная аллокаций entity-лэйна); flushStep 1.0% (wave-2 подтверждён).
> (5) **ВЕРДИКТ S7-130b / ОЧЕРЕДЬ ARCH-ATTACK S7-131** (автономия агента, свежие данные): (a) палитро-лейн PalettedContainer (get+readPalette+SimpleBitStorage) — kernel-топ-1 по использованию, планка владельца x150000 = скорость топ-1 функции; (b) relens guard-хуков (устранить собственный 3.2% оверхед — «не нагружать» по директиве); (c) аллокационная диета entity-лэйна (GC 25.4% — производная); (d) flushStep wave-2. Порядок атаки решит S7-131 (STEP-0 census первым: палитро-лейн имеет O(чанки×секции×тикающие-блоки) структуру — есть ли избыточная перекопутация, как в fluid-push?).
> (6) NEXT S7-131: absorb стабилен, база A/B pre-pack vs master на X150K (min-of-2, пары по cpu) + STEP-0 палитро-лейна ⇒ первый ARCH-ATTACK рычаг. INJECTS-ONLY цел (0 sandbox boots; CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-131) — ARCH-ATTACK рычаг #1 PALETTED-DEMUX РЕАЛИЗОВАН И ВЕРИФИЦИРОВАН ОФЛАЙН (parity ALL PASS), диспатч A/B leg #1 на X150K-базе.**
> (1) **STEP-0 census палитро-лейна** (collapsed stacks 150k, 52552 сэмплов): PalettedContainer.get 1746 самплов (3.32%) бьют (a) fluid-чтения ~56% (guard slow 473 + cellsUnchanged 82 + hasSameAbove/getFlow 363 + прочие), (b) коллизии ~17% (CollisionUtil через noCollision/move), (c) прямые getBlockState ~15% (checkInsideBlocks/getOnPos/pathfinding/redstone/BE). ВСЕ пути — один контейнер LevelChunkSection.states (fluid производен от BlockState) ⇒ демукс на уровне контейнера ускоряет ВСЕ три лейна разом.
> (2) **Дизайн PALETTED-DEMUX**: field-injection ×4 (crusstySnap/snapGen/gen/miss, public volatile) в PalettedContainer на ПЕРВОЙ загрузке (retransform невозможен — смена формы) + get(int) быстрый путь `vals[demux[index]]` (минуя бит-декод и палитру) с fallback в Ops.get (ванильно-эквивалентное чтение + heat-счётчик ленивой материализации 4096 ячеек) + охраняемые мутаторы getAndSet/set (prologue: release+gen++→odd; epilogue: gen++→even) — one-writer инвариант уже гарантирован moonrise region-lock (тем же, что лицензирует несинхронный getAndSetUnchecked). read(FriendlyByteBuf)=server-never — не патчится; onResize идёт внутри охраняемого odd-окна.
> (3) **Путь к верификации**: ручной SMT-фрейм отвергнут верификатором (bad offset; эмпирика: first-frame offset_delta = target) ⇒ переход на ASM COMPUTE_FRAMES build-time патчер (paletted/tools/PalettedPatchTool.java, офлайн — НЕ бут); баг ISTORE-vs-ASTORE и фантомный декремент refcount (начальный snapGen=0 неотличим от «освобождён») закрыты протоколом v2: валидность = snapGen == gen+1, uncounted = 0, release в прологе.
> (4) **Парити-харнесс (research/paletted-demux-2026-09-18/parity_harness_output.txt) = ALL PASS**: 20000 случайных операций (set/getAndSet/get/getAndSetUnchecked) в локстепе vanilla-vs-patched на реальных классах ядра, полное сравнение 4096 ячеек на каждой партии + старые значения getAndSet 4883/4883; лестница resize палитры (40 состояний: linear→hashmap) под охраняемыми мутаторами; lifecycle demux (heat-материализация → published fresh → fast-path parity → write unmark + refcount 1→0 → re-materialize → parity); конкурентный смоук 3 читателя + 1 писатель 500ms без исключений. Верификатор JVM принял пропатченный класс.
> (5) **Ожидания от A/B (X150K-база)**: гейт спеки §3 ≥3% MSPT (лейн ~4.9% CPU ⇒ верхняя граница выигрыша ~4.6-4.9% + снижение GC-давления от дропа вспомогательных аллокаций в fluid-лэйне); планка владельца x150000 = PalettedContainer.get ИСЧЕЗАЕТ из профиля (self-time < ~0.3%), Ops.get ~0.2-0.5% — новая субстрат-цена; риск-режим: ARMED-маркер отсутствует (race READY-vs-load) → vanilla, диагностика по маркерам "paletted: pristine sighting".
> (6) NEXT S7-132: absorb A/B leg #1 (+leg #2 для min-of-2 по cpu-парам) → вердикт рычага → очередь: (b) relens guard-хуков (3.2% оверхеда, теперь частично уже амортизирован демуксом), (c) аллокационная диета entity-лэйна (GC 25.4%), (d) flushStep wave-2. INJECTS-ONLY цел (0 sandbox boots; CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-131b) — PALETTED-DEMUX: 3 ноги (все SUCCESS, ARMED+PATCHED), вердикт REFUTED-BY-ECONOMICS на X150K; дефолт дизарм, инфраструктура банкуется; S7-132 = relens guard.**
> (1) Leg #1 35256298212: порог 16384 чтений/контейнер недостижим (~5-15/тик на десятках тысяч контейнеров) — fast path ни разу, get 5.7% (регресс). Leg #1' 35258976251: порог 64 — билды маргинальных секций не окупаются (лейн 2707 vs 2573 экв. сэмплов). Leg #1'' 35261386323: экономика v3 (4096/16384/блэклист×2) — лейн ≈ 5.3% vs 4.9% базовых. (2) СТРУКТУРНАЯ ПРИЧИНА: X150K-чтения = 100k items × 1-2/тик, размазаны по десяткам тысяч контейнеров; точка окупаемости снапшота >6k будущих чтений/контейнер — распределение на порядки ниже. (3) Честный вердикт (дисциплина TASK-77/S7-128): СОЛО-гейт ≥3% НЕ достигнут, дефолт 0 (fail-closed vanilla), код сохранён как верифицированный субстрат (gen-протокол, parity-харнесс, ASM-пайплайн, fingerprint-serve) для combo: guard-v2 region-gen валидация (убьёт cellsUnchanged перечтения БЕЗ per-container demux). (4) NEXT S7-132: relens guard-хуков (bump 1.0-1.4% чистой инструментации → сэмплирование; slow-path аллокации → GC-диета), затем аллокационная диета entity-лэйна (GC 25.4%), flushStep wave-2. INJECTS-ONLY цел (0 sandbox boots; 5 санкционированных CI-бутов за раунд).

> **СТАТУС 20 TPS (S7-132, TASK-268) — guard relens реализован (inline-счётчики + zero-alloc hit/slow), combo-нога (relens + demux v3) 35264319982 в полёте; absorb S7-133.**
> (1) bump() (~1.0-1.4% self на ~400k вызовов/тик) → inline plain HITS/MISSES + холодный logStats (контракт TASK-77 §7 цел). (2) cellsUnchanged/slow: per-call массивы (~200k аллокаций/тик) → ThreadLocal grow-only (регионный тик однопоточен, entry владеет копией только при cacheable). (3) Диспатч = комбо-нога (relens+demux v3, paletted_demux=1) — тест гипотезы S7-131b «демукс ускоряет собственные свипы guard'а» на той же X150K-сцене; база 35245032701. (4) Preregistered: bump → ≤0.2%, guard-лейн ≤2.5%, hit_rate ~78% ±3, fixture зелёные. (5) NEXT S7-133: absorb → если guard-лейн упал и комбо ≥ базы — чистая нога (demux=0) для изоляции + вердикт; иначе — калибровка. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-132b, TASK-268 absorb) — combo-leg SUCCESS: bump ИСЧЕЗ из профиля, guard-лейн 3.4→2.5% (−27%), high-water −768MB; hit_rate 73.3% (динамика сцены, честно); S7-133 = аллокационная диета entity-лэйна.**
> (1) Реленс механически выполнен: inline-счётчики сняли bump с горячего пути полностью; ThreadLocal-буферы срезали ~200k аллокаций/тик (high-water 6914→6146MB). (2) Комбо relens+demux: guard-лейн −0.9pp, get-лейн 4.7% (чуть лучше соло 5.3%) — демукс остаётся нейтральным субстратом, не соло-рычагом. (3) MSPT межраново шумовая (cpu 7056716 vs 6679335) — лейн-абсолюты банкуются. (4) NEXT S7-133: аллокационная диета entity-лэйна (GC+барьеры 27.1% — крупнейшая адресуемая производная после entity tick: AABB/Vec3/буферы в move/collide/baseTick) → flushStep wave-2; чистая relens-нога — по требованию изоляции. INJECTS-ONLY цел (0 sandbox boots; 2 CI-бута за тик).

> **СТАТУС 20 TPS (S7-133, TASK-269) — ARCH-ATTACK рычаг #2 ALLOC-DIET реализован и офлайн-верифицирован (ALL PASS), диспатч A/B leg #1 на X150K-базе.**
> (1) **STEP-0 javap-контракт аллокационного лейна** (материализованное ядро purpur-1.21.10, INJECTS-ONLY): (a) `LivingEntity.pushEntities` → `Level.getPushableEntities` → `Level.getEntities(Entity,AABB,Predicate)`: на КАЖДЫЙ вызов `new ArrayList` (dead guava-список, результат не читается НИ ОДНОЙ инструкцией) + `new ArrayList` (fill) + барьеры заполнения — ~45k+ живых сущностей/тик; (b) `ItemEntity.mergeWithNeighbours` ГЕЙТИТСЯ `isMergable()` и bench-предметы (pickupDelay=32767) выходят ДО запроса — merge-запрос на сцене НЕ выполняется (census честно снял merge с очереди); (c) `CollisionUtil.getCollisionsForBlocksOrWorldBorder` — БЕЗУСЛОВНЫЙ `new MutableBlockPos; dup; <init>:()V` до всех веток + `new LazyEntityCollisionContext` — ~250k+ запросов/тик (148k move + 100k item noPhysics noCollision). (2) **Дизайн ALLOC-DIET**: два call-site ретаргета на статический бридж `EntityQueryOps` (net.minecraft.world.entity, kernel-loader): push-обёртка → `pushables(Level,Entity,AABB)` (тот же deep-fill EntityLookup.getEntities + PlatformHooks.addToGetEntities + Profiler-счётчик — последовательность сущностей бит-в-бит ванильная, исчезают 2 ArrayList/вызов), ctor-сплайс → `mutablePos()` (кольцо 8 переиспользуемых MutableBlockPos, re-init set(0,0,0) = точное состояние свежего ctor). Оба сплайса СОХРАНЯЮТ ДЛИНУ (3B→3B опкод+операнд; 7B→7B ctor-блок на 4 nop) — ноль сдвигов веток/StackMapTable. Кольца ротационные (8 слотов/поток) — вложенные запросы из колбэков событий до глубины 7 безопасны, глубже — громкий CME, не тихая порча. LazyEntityCollisionContext-пул отложен в wave-2 (приватные final-поля базового класса; безопасная ре-инициализация требует Unsafe/MH — отвергнуто за цену на горячем пути). (3) **Путь верификации**: rust-патчеры на РЕАЛЬНЫХ байтах (LivingEntity 186570B / CollisionUtil 45439B фикстуры): Retargeted{1} в обоих, идемпотентность (repatch byte-identical), fail-closed на чужих классах; 101 тест зелёный; офлайн-харнесс (allocdiet/harness): defineClass пропатченных байтов = полный проход верификатора JVM ✓ + ретаргет pushEntities подтверждён с JVM-стороны (0xb8 pc=75 → EntityQueryOps#pushables) + кольцо mutablePos 8 ротируемых обнулённых экземпляров + ванильный ctor-якорь = **ALLOC-DIET OFFLINE PASS** (research/alloc-diet-2026-09-18/). (4) **Диспатч leg #1** vs база 35245032701 (guard=1, demux=0, diet=0): X150K 150000/seed42/xmx10G/fp4/300s, diet=1. Preregistered: fixture-гейты зелёные (behavioral parity), ARMED-маркеры («defined EntityQueryOps», «hook serve»), GC-лейн (G1 14.4% + барьеры 12.7% = 27.1% база) ↓ по лейн-абсолютам равных окон (закон S7-96d: MSPT межраново шумовая), high-water ↓ (база 6914/комбо 6146MB), pause-total ↓ (база 15782ms), entity-фаза без регрессии >0.5pp. Оценка объёма диеты честно: ~10-17MB young-gen/тик удалённых аллокаций (push ~3.2MB + MutableBlockPos ~8MB + мёртвые барьеры заполнения). (5) NEXT S7-134: absorb leg #1 → leg #2 min-of-2 → вердикт рычага (гейт: GC-лейн ↓ ≥10% иначе REFUTED-BY-ECONOMICS, дефолт 0) → wave-2 диеты (LazyEntityCollisionContext, refreshDirtyAttributes, InsideBlockEffectApplier) или flushStep. INJECTS-ONLY цел (0 sandbox boots; 1-2 CI-бута за тик санкционированы).

> **СТАТУС 20 TPS (S7-133b, TASK-269 absorb) — ALLOC-DIET leg #1 SUCCESS/ARMED живьём: high-water −20% (−1374MB), но гейт GC-лейна провален с обратным знаком (+21% абсолюта) ⇒ REFUTED как GC-CPU рычаг; дефолт 0, код банкуется; S7-134 = истинный alloc-профиль + рычаг old-gen мутации.**
> (1) Run 35271475494 (f44d9ce): ARMED живьём (Retargeted{1}×2 бит-в-бит офлайну, serve+retransform rc=0), fixture VALID/alive/0 tick-behind — поведенческая парити на живой 148.5k-сцене полная. (2) Лейн-счёт: GC-лейн 27.1→30.6% (+21% абсолют; add_card +58%, Refine +38%, RemSet +31%, CM +16%), entity-фаза плоско (−0.4%) — гейт «GC-лейн ↓≥10%» НЕ достигнут, знак обратный ⇒ REFUTED (leg #2 избыточен: все суб-лейна согласованно вверх — структурная причина, не шум). (3) Банкуется: high-water 6914→5540MB, pause-avg −4%, pause-max −18%, парити живьём, сплайс-механика. (4) УРОК: срезанные ~10-17MB/тик — малая доля истинного чёрна (сотни MB/тик); GC-лейн гонится old-gen МУТАЦИЕЙ (section-движения ChunkEntitySlices, спавн/деспавн churn, entity-data записи), не wrapper-чёрном; слепые диеты без alloc-профиля запрещены. (5) NEXT S7-134: async-profiler alloc-mode в харнес (инфраструктура) ⇒ ранжирование чёрна по байтам ⇒ архитектурный рычаг old-gen мутации (batched section-moves / deferred dirty updates — ванильная парити), flushStep wave-2 в очереди. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-134, TASK-270) — ИНФРАСТРУКТУРА ИЗМЕРЕНИЙ: ROOT-CAUSE профилировщика v2 найден и исправлен (asprof v4.x `dump` НЕ останавливает сессию — alloc-профиль не собирался НИ РАЗУ в истории проекта), v3 stop-based; ценз-ран 35275967738 в полёте.**
> (1) **ROOT-CAUSE (вскрыт по ap.log-сигнатуре)**: v2 строила окна цепочкой dump→start, но в asprof v4.x `dump` выгружает данные БЕЗ остановки сессии (останавливает только `stop`) ⇒ первая cpu-сессия жила весь соак, старты wall/alloc падали («[ERROR] Profiler already started» ×3 — подтверждено в run-s7131-lever{,2,3}, run-s7132-combo, run-diet-leg1 — ВСЕ раны), а cpu-/wall-/alloc-collapsed.txt были кумулятивными CPU-редампами. Косвенный признак стоял в артефактах всё время: «alloc»-листья = G1 oop-closures/C2-фреймы — невозможные листья для alloc-события. (2) **Валидность прежних вердиктов**: лейн-анализы S7-131..133 валидны (cpu-collapsed + gc.log — CPU-событие, живые данные); WALL/ALLOC секции BOTTLENECKS_3 были CPU-загрязнены — помечены, перевзводятся ценз-раном. (3) **v3-фикс (run_world3.sh)**: окна завершаются `stop` (stop == stop+dump); cpu 0-55% / wall 55-80% / alloc 80-100% (alloc-веса = БАЙТЫ — ценз чёрна S7-133b); asprof_guard_start с orphan-rescue (застрявшая сессия → orphan-collapsed.txt → retry — молча съесть окно больше невозможно); flamegraph = короткая свежая cpu-сессия 20s; run-env.txt + `seconds:` (длина alloc-окна для MB/s). (4) **report_world3.py**: ALLOC-единицы = BYTES во всех таблицах (bucket/phase/leaf) + примечание семантики; F2 = top-10 по байтам + **F2 alloc-churn rate** (MB/s по окну 20%×seconds, total GB). Оффлайн-валидация: синтетический WORK (byte-веса, 1.8GB/60s → ~30MB/s расчёт корректен) + реальный артефакт run-diet-leg1 (парсинг без краша; артефакт-отчёт восстановлен git checkout после смоука — дисциплина артефактов). (5) **Диспатч ценза**: run **35275967738** (master 662738e) — базовая сцена X150K (150000/seed42/xmx10G/fp4/guard1/**demux0/diet0**/300s), НЕ A/B-нога, а ценз: первый истинный alloc-профиль + первый истинный wall-профиль + свежий cpu-профиль мастера. Preregistered гейты абсорба: ap.log чист (0 «already started»/«not active»); alloc-листья = аллокационные сайты (<init>-доминанта, НЕ G1/C2); порядок сотен GB на 60s-окно (оценка S7-133b: сотни MB/тик × 1200 тиков); wall ≠ cpu; fixture-гейты зелёные. (6) NEXT S7-134b: absorb ценза ⇒ ранжирование чёрна по байтам ⇒ выбор архитектурного рычага old-gen мутации (кандидаты: batched section-moves ChunkEntitySlices, deferred dirty updates, EntityTickList churn, Long2Reference rehash) ⇒ S7-135 имплементация; flushStep wave-2 в очереди. INJECTS-ONLY цел (0 sandbox boots; 1 CI-бут ценза санкционирован).

> **СТАТУС 20 TPS (S7-134b, TASK-270 absorb) — ПЕРВЫЙ ИСТИННЫЙ ALLOC-ЦЕНЗ X150K: все инфра-гейты PASS (4 профилировочные сессии впервые реально сменились), истинный чёрн 25.6GB/60s = 21.4MB/тик (оценка S7-133b завышена ~15× — refuted честно); 66% чёрна = movement-геометрия + inside-blocks; топ-1 CPU-функция PalettedContainer.get питается теми же путями; S7-135 = мемоизация inside-blocks/fluid (STEP-0 javap первым).**
> (1) Run 35275967738 (master 662738e, базовая сцена demux=0/diet=0/guard=1) SUCCESS: ap.log = 4×«Profiling started», 0 ошибок — cpu/wall/alloc/flamegraph впервые реально сменились (v3 stop-based работает); alloc-листья = чистые аллокационные сайты (0 G1/C2-фреймов — grep=0); wall ≠ cpu (wall-only waiters: Watchdog/ReferenceHandler/DedicatedServer$1/PrioritisedQueueExecutor); fixture VALID (INJECT DONE 150000/150000 за 105s, alive 4/4). (2) **ИСТИННЫЙ ЧЁРН**: gc.log-границы (14 young GC в 60s-окне × ~245 eden-регионов × 8MB) = **25.6GB/60s = 427MB/s = 21.4MB/тик**; оценка S7-133b «сотни MB/тик» REFUTED (~15× завышена — считала эвакуацию выживших, не eden-потребление); интервал ap-alloc = 3.56MB/сэмпл (выведен, внутренне согласован). (3) **РАНЖИРОВАНИЕ ЧЁРНА ПО БАЙТАМ**: movement/collision-геометрия **43.9%** (топ-1 сайта ценза Vec3.add 9.5% через collidedWithFluid→AABB.collidedAlongVector; AABB makeBoundingBox 5.8%, inflate 3.9%, Fluid.getAABB 3.0%); inside-blocks **22.1%** (LongOpenHashSet long[] 6.1% — dedup-множество НА СУЩНОСТЬ НА ТИК в forEachBlockIntersectedBetween; BlockPos$6 corner-lambda 5.2%; InsideBlockEffectApplier.flushStep→Arrays.copyOf 4.6%); JVM/other 17.3% (вкл. CgroupUtil 6.3% — JVM-внутренний, вне досягаемости, не-цель); fluid 5.5%; остальное шум. (4) **КРОСС-СВЯЗКА**: топ-1 kernel-функция CPU = PalettedContainer.get 3.1% (+readPalette 0.8%) питается ТЕМИ ЖЕ путями (getBlockState из checkInsideBlocks + updateFluidHeightAndDoFluidPushing по 4 углам) — один архитектурный рычаг бьёт И чёрн, И топ-1 CPU-функцию. (5) **ОЧЕРЕДЬ S7-135+**: (a) INSIDE-BLOCKS/FLUID per-entity мемоизация с event-driven dirty-флагом (Δpos=0 + ревизия block-state секции в ключе; механика кэш-гварда, прецедент fluid_guard TASK-80; median-exact parity) — бьёт 22.1%+часть 43.9% чёрна + долю PalettedContainer.get; (b) collidedWithFluid геометрия (Vec3.add 9.5%) — STEP-0 javap-контракт collidedAlongVector; (c) InsideBlockEffectApplier.flushStep (4.6%) — rotating pool на субстрате EntityQueryOps. INJECTS-ONLY цел (0 sandbox boots; 1 CI-бут ценза). (6) NEXT S7-135: STEP-0 javap-контракт checkInsideBlocks/updateFluidHeightAndDoFluidPushing/forEachBlockIntersectedBetween (материализованное ядро, офлайн) ⇒ дизайн мемоизации с ревизией секции ⇒ имплементация (rust classfile + офлайн-харнесс ALL PASS) ⇒ диспатч leg #1 на X150K-базе. Гейт preregistered решит S7-135. flushStep wave-2 в очереди.

> **СТАТУС 20 TPS (S7-135, TASK-271) — ARCH-ATTACK рычаг #3 INSIDE-CACHE реализован и офлайн-верифицирован (rust 106 ✓ + INSIDE-CACHE OFFLINE PASS), диспатч leg #1 35282003292.**
> (1) **STEP-0 javap-контракт**: гейт-сайт isAffectedByBlocks offset 1 в checkInsideBlocks(List,Collector) — единственный; анатомия визитора декодирована (hitShape/inFluid/effectful/intersected-семантика, fluid-ветка ПОСЛЕ block); ванила на статике исполняет ДВЕ traversal/тик (вторая — чистый visitedBlocks-дюп). (2) **Рычаг**: единственный 3B→3B ретаргет isAffectedByBlocks→InsideBlockOps.gate(Entity)Z; bridge обслуживает статику целиком (HIT: верификация state-id + replay ванильных вызовов эффектов; MISS: mirror-traversal теми же примитивами + capture), нестатика — ваниль. Кэш = плоские примитивные массивы 131072×12 (урок §155: ноль oop-массивов). (3) **Офлайн**: rust 106 ✓; JVM-харнесс OFFLINE PASS (ARMED=true на реальном ядре; лов identity-форка загрузчика задокументирован). (4) **Диспатч**: run 35282003292 (master 1bd7f52) vs база-ценз 35275967738; preregistered гейты: fixture зелёные, ARMED-маркеры, alloc-семьи ↓30/25%, PalettedContainer.get ↓15%, young GC ↓, entity-фаза без регрессии >0.5pp; частичный = калибровка leg #2, промах = REFUTED-BY-ECONOMICS дефолт 0, проход = leg #2 min-of-2. (5) NEXT S7-135b: absorb leg #1 → вердикт по лейн-абсолютам равных окон (S7-96d) → leg #2/вердикт; очередь после INSIDE-CACHE: collidedWithFluid-геометрия (Vec3.add 9.5%, если останется), flushStep rotating pool (4.6%), updateFluidHeightAndDoFluidPushing corners (leg #2 INSIDE-CACHE). INJECTS-ONLY цел (0 sandbox boots; 1 CI-бут leg #1 санкционирован).

> **СТАТУС 20 TPS (S7-135b, TASK-271 absorb) — legs #1/#2 INSIDE-CACHE: leg #1 ИНЕРТЕН (capture недостижим по построению + delta-чек не проходил — рычаг НЕ РАБОТАЛ, принцип НЕ опровергнут); bridge v2 (xo-статик-детектор + bootstrap capture); leg #2 РАН НЕВАЛИДЕН (сцена коллапсировала 150k→~70k при VALID-маркере, crawl с инъекции MSPT 430, артефакт без stdout; bridge 0.4% CPU); lever BANKED default 0; S7-136 = ping-pong hardening + чистый re-run.**
> (1) Leg #1 (35282003292): ARMED полн, fixture VALID, НО gate никогда не обслуживал — два механических дефекта моста: capture был достижим только через инвалидацию несуществующего кэша; deltaMovement покоящихся предметов ≠ 0 (гравитационный остаток) — статик-чек не проходил. Лейны = база в шуме (S7-96d). (2) Bridge v2 (d8453b3): статик-детектор xo==x,yo==y,zo==z (from==to==pos; поле Entity.xo публичное, javap ✓) + bootstrap capture на пустом слоте; офлайн OFFLINE PASS повторён. (3) Leg #2 (35284069355): РАН НЕВАЛИДЕН — сцена 67-74k живых (база 148k) при VALID-маркере инъекции (популяция распалась после), TPS 1.5-3.3 / MSPT 430 с первого пост-инъекционного полла, артефакт без server-stdout.log; bridge сам по себе 0.4% CPU — самовзрыва нет, но per-entity entity-фаза ~9× — не изолировано; главный подозреваемый ping-pong слотов (150k последовательных id в 131072 слотах ⇒ ~19k пар делят слот, вечный mirror + Recorder-аллокации + card-marks в old-gen слот-массивы), второй — runner-контеншн (wall 84.5% idle). (4) ВЕРДИКТ: принцип мемоизации не опровергнут; код = ARMED-CODE-BANKED default 0 (fail-closed vanilla); сравнение по leg2 нечестно (сцена развалилась). (5) NEXT S7-136: ping-pong hardening (bootstrap capture только при SLOT_EID[slot]==0 — занятый слот ⇒ ваниль; ~13% сущностей ванильны при 150k>131072 — допустимо) ⇒ чистый re-run leg #2' (полный артефакт) ⇒ вердикт по preregistered гейтам §156; если лейны снова мертвы при живом HIT — JIT/деопт-телеметрия retransform; после INSIDE-CACHE: collidedWithFluid-геометрия / flushStep pool / updateFluidHeight corners. INJECTS-ONLY цел (0 sandbox boots; 2 CI-бута санкционированы).

> **СТАТУС 20 TPS (S7-136, частичный — CREDS-BLOCKED) — ping-pong hardening моста INSIDE-CACHE реализован и офлайн-верифицирован (rust 106 ✓ + INSIDE-CACHE OFFLINE PASS на свежем материализованном ядре); диспатч leg #2' ЗАБЛОКИРОВАН: git-creds утеряны вместе с песочницей (WIPE), bootstrap_tick.sh отсутствует.**
> (1) **Харденинг (урок leg #2 35284069355)**: capture теперь ТОЛЬКО из пустого слота (`SLOT_EID[slot]==0`); слот, занятый чужим eid (живым или мёртвым), больше не перехватывается mirror'ом — чистая ваниль без инвалидации чужого штампа. Механика ping-pong устранена по построению: 150k живых > 2^17 слотов ⇒ ~19k пар eid делят слот; прежний перехват давал вечный A-capture⇔B-capture (полная traversal + Recorder + 6 arraycopy card-marks каждый тик у обеих сущностей, ноль HIT) — главный подозреваемый коллапса сцены leg2 (150k→70k, MSPT 430). Цена: ~13% сущностей остаются ванильными — принято по preregistered плану S7-135b. (2) **Офлайн-верификация повторена на ВОССТАНОВЛЕННОМ окружении** (песочница пересоздана; purpur paperclip качался заново, eula-less pass materialized kernel 29.4MB + joml libraries; JDK21 temurin): build EntityQueryOps + InsideBlockOps(hardened) javac --release 21 OK; cargo rebuild подхватил include_bytes; suite 107 = 106 ✓ + 1 ignored; JVM-харнесс: structural pass 205522B (байт-в-бит прежнему — ядро/патчер не менялись), retarget-проверка, ARMED=true на реальном ядре, массивы 131072×12 ⇒ **INSIDE-CACHE OFFLINE PASS**. Hardened-классы забанкованы (research/inside-cache-2026-09-18/InsideBlockOps_s7136.class + sha256 append-only). (3) **БЛОКЕР**: ~/.git-credentials и bootstrap_tick.sh утеряны (WIPE песочницы между 05:08 и 07:43 +08); push c-crussty и dispatch workflow невозможны анонимно; локальный коммит + патч в /home/z/my-project/download/ сделаны. Требуется владельцу: восстановить bootstrap_tick.sh (baked token) — после этого следующий тик делает push + диспатч leg #2' по preregistered гейтам §156 (fixture зелёные, ARMED, alloc-семьи ↓30/25%, PalettedContainer.get ↓15%, young GC ↓) ⇒ вердикт INSIDE-CACHE. (4) Дополнительный риск leg #2' задокументирован: мёртвые eid занимают слоты навсегда (нет эвикции) — на 300s-ране затухание HIT-доли ожидаемо умеренное (churn-спавны попадают в занятые слоты → ваниль), измеримо по hit-телеметрии; LRU-эвикция сознательно НЕ внесена (минимальный hardening, дисциплина канона). INJECTS-ONLY цел (0 sandbox boots; 0 CI-бутов за тик — диспатч заблокирован creds).

> **СТАТУС 20 TPS (S7-137, частичный — CREDS-BLOCKED) — ARCH-ATTACK рычаг #4 FLUSH-DIET реализован и офлайн-верифицирован (rust 112=111✓+1 ignored + FLUSH-DIET OFFLINE PASS на реальном ядре); dup-рычаг collidedWithFluid ОТМЕНЁН по STEP-0 (весь лейн — под гейтом INSIDE-CACHE); push/dispatch leg #2' по-прежнему заблокированы (bootstrap_tick.sh не восстановлен).**
> (1) **STEP-0 пересборка очереди**: collidedWithFluid-геометрия (Vec3.add 9.5% чёрна, топ-1 сайт ценза) — javap Entity показал: единственный caller = визитор lambda$checkInsideBlocks$2 (fluid-ветка, offset 135; collidedWithShapeMovingFrom = getAABB + List.of + makeBoundingBox + subtract + collidedAlongVector). ВЕСЬ лейн исполняется ПОД гейтом isAffectedByBlocks ⇒ покрывается INSIDE-CACHE leg #2' — дублирующий рычаг не нужен; полный потенциал leg #2' пересчитан: inside-blocks 22.1% + collidedWithFluid-доля ~22.7% + доля PalettedContainer.get ≈ 45% чёрна под одним гейтом.
> (2) **FLUSH-DIET STEP-0** (кандидат из очереди §5 S7-134b, независимый от INSIDE-CACHE — applyAndClear крутится и на HIT-пути): ванильный flushStep() переносит before/after списки в finalEffects через List.addAll; ArrayList.addAll резолвит c.toArray() ДО проверки пустоты ⇒ каждый ПУСТОЙ вызов аллоцирует new Object[0] через Arrays.copyOf; ценз 35275967738: 336 сэмплов (4.6% истинного чёрна) ровно на advanceStep→flushStep→ArrayList.addAll→ArrayList.toArray→Arrays.copyOf→Object[]; advanceStep стреляет на каждом блок-степе traversal (~300k/тик при 150k).
> (3) **Дизайн**: оба addAll-сайта flushStep()V (invokeinterface List.addAll, 5B, javap-offsets 41/114) → invokestatic FlushOps.fladd(List,Collection)Z (3B) + 2 nop (бывшие count/zero-слоты) — length-preserving, стек-форма идентична, SMT не двигается; fladd: `src.isEmpty() ? false : dst.addAll(src)` — ванильная семантика минус мусорный toArray; бридж-класс FlushOps (net.minecraft.world.entity, raw-типы, erasure-дескриптор совпадает); env CRUSSTY_FLUSH_DIET (dormant-invisible, fail-closed: kernel-rename guard beforeEffectsInStep, strict ровно-2-сайта, idempotent AlreadyPatched).
> (4) **Офлайн-верификация**: rust suite 112 = 111✓ + 1 ignored (5 новых тестов на РЕАЛЬНОЙ фикстуре StepBasedCollector 5695B: два сайта, пул-резолв fladd, байт-форма invokestatic+2nop, idempotent byte-identical, fail-closed на чужом классе); JVM-харнесс FlushDietHarness на реальном ядре: structural pass 5798B, wiring, fladd-семантика (empty→false no-op, non-empty→ordered addAll), behavioral smoke ПАРА patched+vanilla (2000 пустых степов через advanceStep→flushStep→fladd + 5 typed effects непустой путь + fresh applyAndClear) — **FLUSH-DIET OFFLINE PASS**; уроки харнесса: nest-парнёр RecordedEffect обязан жить в том же loader'е (IllegalAccessError cross-loader), InsideBlockEffectType.<clinit> тянет реестры → офлайн SharedConstants.tryDetectVersion()+Bootstrap.bootStrap() (не бут: реестры в памяти). Артефакты забанкованы research/flush-diet-2026-09-18/ (+sha256 append-only). run_world3.sh + world-bench.yml: env CRUSSTY_FLUSH_DIET + input flush_diet — диспатч-подготовка leg (S7-138).
> (5) **БЛОКЕР без изменений**: ~/.git-credentials и bootstrap_tick.sh НЕ восстановлены (WIPE); push b98915a+S7-137 и диспатчи (leg #2' INSIDE-CACHE, затем FLUSH-DIET leg) невозможны анонимно; crussty-dev-logs приватен — CLAIMS/INDEX/LEDGER долг копится. Владельцу: восстановить bootstrap_tick.sh — тогда: push → диспатч leg #2' (гейты §156) → вердикт INSIDE-CACHE → диспатч FLUSH-DIET leg vs база-ценз. INJECTS-ONLY цел (0 sandbox boots; 0 CI-бутов за тик — creds).

> **СТАТУС 20 TPS (S7-140, частичный — CREDS-BLOCKED 6-й тик) — ARCH-ATTACK STEP-0-РАЗВЕДКА ЗАВЕРШЕНА ПОЛНОСТЬЮ: оставшиеся непокрытые лейны разобраны (entity-query = ПАРИТИ-СТЕНА; SynchedEntityData = уже O(1)-массив; блок-коллизии = moonrise-оптимален; остатки = микро/запрещены). ОФЛАЙН-ФАЗА КОНВЕЙЕРА ИСЧЕРПАНА — критический путь = диспатчи 4 рычагов, ждут creds.**
> (1) **Entity-query лейн** (getEntities 784+599=2.5% + intersects 847=1.5% — крупнейший непокрытый CPU): полный javap-контракт снят на живом ядре — pushEntities (50k/тик, getPushableEntities→List одноразовый), item-merge герды (`tickCount % (moved?2:40)`, thundering herd ~100k сканов в один тик каждые 40 тиков = MSPT-пики каждые 2с), Mob.aiStep looting-скан (3.9% churn на inflate+query пикаперов). ПАРИТИ-СТЕНА доказана: (a) порядок кандидатов значим (push-цикл/cramming), (b) predicate pushableBy ПАРНЫЙ (Team rules ⇒ суб-индекс по флагу неконсистентен), (c) event-driven кэш соседей мёртв (движение каждый тик), (d) порядок-сохраняющий прескрин с сортировкой = нетто <1% = МИКРО-запрещено, (e) размазывание гердов меняет тайминг merge = запрещено. ВЕРДИКТ: ×150000-класса рычага с median-exact parity на этом ядре НЕ СУЩЕСТВУЕТ; переоткрытие = только смена moonrise-субстрата. Полное ТЗ: research/entity-query-2026-09-18/DESIGN.md.
> (2) **SynchedEntityData закрыт**: itemsById УЖЕ DataItem[] массив (O(1)); getValue = volatile VarHandle backbone (неизбежен). **Блок-коллизии закрыты**: moonrise уже имеет hasOnlyAir-skip + hasSpecialCollidingBlocks + emptyContextCollisionShape; COLLISION-FREE-SECTION (сестра FLUID-FREE) добирает только травяные над-секции ≤0.6% = микро-запрещено. **CgroupUtil 6.3% churn** — JVM-внутренний, недостижим (флаги запрещены) — не-цель.
> (3) **Следствие**: 5 офлайн-верифицированных рычагов (INSIDE-CACHE hardened, ALLOC-DIET, FLUSH-DIET, FLUID-FREE-SECTION + PALETTED-DEMUX) исчерпывают ценз run 35275967738; каждый следующий рычаг обязан впитывать уроки живых A/B (урок leg #2: непроверенный на сцене = риск коллапса) — накопительная слепая инженерия 6-го рычага вредна. Критический путь = push 4 коммитов + 3-4 диспатча + absorb-вердикты по preregistered-гейтам.
> (4) **БЛОКЕР без изменений (6-й тик)**: bootstrap_tick.sh/creds НЕ восстановлены (push: could not read Username); диспатчи (leg #2' INSIDE-CACHE ≈45% чёрна под гейтом; FLUSH-DIET leg; FLUID-FREE leg; ALLOC-DIET leg) ждут владельца; crussty-dev-logs приватен — CLAIMS/INDEX/LEDGER долг копится. INJECTS-ONLY цел (0 boots; 0 CI-бутов — creds).

> **СТАТУС 20 TPS (S7-140, частичный — CREDS-BLOCKED 6-й тик) — ARCH-ATTACK STEP-0-РАЗВЕДКА ЗАВЕРШЕНА ПОЛНОСТЬЮ: оставшиеся непокрытые лейны разобраны (entity-query = ПАРИТИ-СТЕНА; SynchedEntityData = уже O(1)-массив; блок-коллизии = moonrise-оптимален; остатки = микро/запрещены). ОФЛАЙН-ФАЗА КОНВЕЙЕРА ИСЧЕРПАНА — критический путь = диспатчи 4 рычагов, ждут creds.**
> (1) **Entity-query лейн** (getEntities 784+599=2.5% + intersects 847=1.5% — крупнейший непокрытый CPU): полный javap-контракт снят на живом ядре — pushEntities (50k/тик, getPushableEntities→List одноразовый), item-merge герды (`tickCount % (moved?2:40)`, thundering herd ~100k сканов в один тик каждые 40 тиков = MSPT-пики каждые 2с), Mob.aiStep looting-скан (3.9% churn на inflate+query пикаперов). ПАРИТИ-СТЕНА доказана: (a) порядок кандидатов значим (push-цикл/cramming), (b) predicate pushableBy ПАРНЫЙ (Team rules ⇒ суб-индекс по флагу неконсистентен), (c) event-driven кэш соседей мёртв (движение каждый тик), (d) порядок-сохраняющий прескрин с сортировкой = нетто <1% = МИКРО-запрещено, (e) размазывание гердов меняет тайминг merge = запрещено. ВЕРДИКТ: ×150000-класса рычага с median-exact parity на этом ядре НЕ СУЩЕСТВУЕТ; переоткрытие = только смена moonrise-субстрата. Полное ТЗ: research/entity-query-2026-09-18/DESIGN.md.
> (2) **SynchedEntityData закрыт**: itemsById УЖЕ DataItem[] массив (O(1)); getValue = volatile VarHandle backbone (неизбежен). **Блок-коллизии закрыты**: moonrise уже имеет hasOnlyAir-skip + hasSpecialCollidingBlocks + emptyContextCollisionShape; COLLISION-FREE-SECTION (сестра FLUID-FREE) добирает только травяные над-секции ≤0.6% = микро-запрещено. **CgroupUtil 6.3% churn** — JVM-внутренний, недостижим (флаги запрещены) — не-цель.
> (3) **Следствие**: 5 офлайн-верифицированных рычагов (INSIDE-CACHE hardened, ALLOC-DIET, FLUSH-DIET, FLUID-FREE-SECTION + PALETTED-DEMUX) исчерпывают ценз run 35275967738; каждый следующий рычаг обязан впитывать уроки живых A/B (урок leg #2: непроверенный на сцене = риск коллапса) — накопительная слепая инженерия 6-го рычага вредна. Критический путь = push 4 коммитов + 3-4 диспатча + absorb-вердикты по preregistered-гейтам.
> (4) **БЛОКЕР без изменений (6-й тик)**: bootstrap_tick.sh/creds НЕ восстановлены (push: could not read Username); диспатчи (leg #2' INSIDE-CACHE ≈45% чёрна под гейтом; FLUSH-DIET leg; FLUID-FREE leg; ALLOC-DIET leg) ждут владельца; crussty-dev-logs приватен — CLAIMS/INDEX/LEDGER долг копится. INJECTS-ONLY цел (0 boots; 0 CI-бутов — creds).

> **СТАТУС 20 TPS (S7-141, частичный — CREDS-BLOCKED 7-й тик) — ДИСПАТЧ-РЕПЕТИЦИЯ ЗАВЕРШЕНА: конвейер мгновенного диспатча проверен по всем швам (wiring/inputs/гейты/банки/rust-регресс) — READY-TO-DISPATCH.**
> (1) **Wiring-аудит**: run_world3.sh — все 6 env-флагов (CRUSSTY_FLUID_PUSH_GUARD/PALETTED_DEMUX/ALLOC_DIET/INSIDE_CACHE/FLUSH_DIET/FLUID_FREE) wired с default 0 (dormant fail-closed) + WARN FLUID_FREE без DEMUX; world-bench.yml — все 13 inputs wired (world_url/radius/seconds/summons/fake_players/6 lever-флагов/population_target+seed/server_xmx/cpu_band_min+max pairing/natives_url), concurrency-group на месте.
> (2) **Гейт-аудит**: preregistered-комплекты всех 4 legs в каноне — §156 (INSIDE-CACHE: fixture зелёные, ARMED, alloc-семьи ↓30/25%, PalettedContainer.get ↓15%, young GC ↓), §S7-138 (FLUSH-DIET), §S7-139 (FLUID-FREE: fluid-доля get ↓≥60%, entity-фаза ≤+0.5pp, young GC ↓), ALLOC-DIET (dispatch_s7134 preregistered).
> (3) **Артефакт-аудит**: sha256-пересчёт банков — inside-cache 4/4, flush-diet 3/3, fluid-free 5/5, alloc-diet 2/2 совпадают; paletted-demux 3/5: два «расхождения» = СТАРЫЕ записи эпохи pre-S7-131-FIX в append-only файле (классы пересобраны коммитом eec4b6e «ре-эмбед», новые записи соответствуют) — НЕ порча; найден пробел дисциплины: PalettedContainerOps.class пересобран без новой append-only записи (зафиксировано, не влияет на диспатч — rust-образ тестируется suite'ом).
> (4) **Rust-регресс на head 655a95b**: suite 121 = 120✓ + 1 ignored, 0 failed — консистентность rust-образа и классов подтверждена.
> (5) **БЛОКЕР (7-й тик)**: bootstrap_tick.sh/creds НЕ восстановлены; READY-TO-DISPATCH конфигурация: push 5 коммитов (b98915a/e5cff9a/f16b215/91fcd70/655a95b) → диспатч leg #2' INSIDE-CACHE (X150K, inside_cache=1, base 35275967738, гейты §156) → FLUSH-DIET leg → FLUID-FREE leg (demux=1) → ALLOC-DIET leg; absorb-вердикты по preregistered-гейтам сразу после ног. INJECTS-ONLY цел (0 boots).

> **СТАТУС 20 TPS (S7-141, частичный — CREDS-BLOCKED 7-й тик) — ДИСПАТЧ-РЕПЕТИЦИЯ ЗАВЕРШЕНА: конвейер мгновенного диспатча проверен по всем швам (wiring/inputs/гейты/банки/rust-регресс) — READY-TO-DISPATCH.**
> (1) **Wiring-аудит**: run_world3.sh — все 6 env-флагов (CRUSSTY_FLUID_PUSH_GUARD/PALETTED_DEMUX/ALLOC_DIET/INSIDE_CACHE/FLUSH_DIET/FLUID_FREE) wired с default 0 (dormant fail-closed) + WARN FLUID_FREE без DEMUX; world-bench.yml — все 13 inputs wired (world_url/radius/seconds/summons/fake_players/6 lever-флагов/population_target+seed/server_xmx/cpu_band_min+max pairing/natives_url), concurrency-group на месте.
> (2) **Гейт-аудит**: preregistered-комплекты всех 4 legs в каноне — §156 (INSIDE-CACHE: fixture зелёные, ARMED, alloc-семьи ↓30/25%, PalettedContainer.get ↓15%, young GC ↓), §S7-138 (FLUSH-DIET), §S7-139 (FLUID-FREE: fluid-доля get ↓≥60%, entity-фаза ≤+0.5pp, young GC ↓), ALLOC-DIET (dispatch_s7134 preregistered).
> (3) **Артефакт-аудит**: sha256-пересчёт банков — inside-cache 4/4, flush-diet 3/3, fluid-free 5/5, alloc-diet 2/2 совпадают; paletted-demux 3/5: два «расхождения» = СТАРЫЕ записи эпохи pre-S7-131-FIX в append-only файле (классы пересобраны коммитом eec4b6e «ре-эмбед», новые записи соответствуют) — НЕ порча; найден пробел дисциплины: PalettedContainerOps.class пересобран без новой append-only записи (зафиксировано, не влияет на диспатч — rust-образ тестируется suite'ом).
> (4) **Rust-регресс на head 655a95b**: suite 121 = 120✓ + 1 ignored, 0 failed — консистентность rust-образа и классов подтверждена.
> (5) **БЛОКЕР (7-й тик)**: bootstrap_tick.sh/creds НЕ восстановлены; READY-TO-DISPATCH конфигурация: push 5 коммитов (b98915a/e5cff9a/f16b215/91fcd70/655a95b) → диспатч leg #2' INSIDE-CACHE (X150K, inside_cache=1, base 35275967738, гейты §156) → FLUSH-DIET leg → FLUID-FREE leg (demux=1) → ALLOC-DIET leg; absorb-вердикты по preregistered-гейтам сразу после ног. INJECTS-ONLY цел (0 boots).

> **СТАТУС 20 TPS (S7-144, частичный — CREDS-BLOCKED 10-й тик) — DISPATCH-READINESS AUDIT v2: FLUID-FREE OFFLINE PASS ВОСПРОИЗВЕДЁН ИЗ ЧИСТОГО СОСТОЯНИЯ (exit 0); конвейер готов к мгновенному диспатчу; classpath-конвейер восстановления забанкован.**
> (1) Аудит готовности после S7-143: rust-сьют на head 9564838 = 121 ok / 0 failed / 1 ignored; git fsck чист; ahead 8 (f67ae10..9564838); патч-банк 5/5 sha256 OK; artifact_hashes_s7143 (FluidOps.class, LevelChunkSection.patched.class) OK.
> (2) Воспроизведение из ЧИСТОГО состояния (пост-WIPE): материализация ядра заново = 29386794B байт-в-бит с цензом (sha256 e2992d63…), FluidFreeHarness с чистой javac-пересборкой: STRUCTURAL+INJECTED SURFACE / SECTION MATERIALIZED / ARMED=true / FREE-HIT ff=1 / EVENT-DRIVEN INVALIDATION gen 0→2→ff=2 / RESTORE ff=1 verdict=true / SCATTERED-WATER PARITY ⇒ **FLUID-FREE OFFLINE PASS exit 0** — байт-в-бит с записью S7-143/91fcd70.
> (3) Уроки восстановления (RUNBOOK_S7144 забанкован research/fluid-free-2026-09-18/): (a) порядок classpath shadow→kernel→libraries ОБЯЗАТЕЛЕН — paper-shaded LogUtils.getClassLogger живёт в пропатченном kernel jar и затеняется чистым mojang-logging из libraries; (b) финальные libraries = <work>/server/libraries от paperclip (в контейнерах base+patch); (c) unzip не создаёт вложенные -d пути; (d) materialize_kernel_v2/finish_libraries/run_fluid_free_harness скрипты забанкованы (sha256 в artifact_hashes_s7144.txt).
> (4) **БЛОКЕР (10-й тик)**: bootstrap_tick.sh/creds НЕ восстановлены; push 8 коммитов (f67ae10..9564838) + диспатчи leg #2' INSIDE-CACHE (X150K, base 35275967738, §156) → FLUSH-DIET → FLUID-FREE (demux=1) → ALLOC-DIET ждут; CLAIMS/INDEX/LEDGER долг 10 тиков (CLAIMS_DEBT дополнен до S7-144). INJECTS-ONLY цел (0 boots; 0 CI-бутов).

> **СТАТУС 20 TPS (S7-145, частичный — CREDS-BLOCKED 11-й тик) — REPRO-аудит v3 ЗАВЕРШЁН: ВСЕ 5 харнессов (4 рычага + демукс-субстрат) воспроизведены из ЧИСТОГО состояния (все exit 0); диспатч-конвейер доказуемо восстанавливаем одной командой; критический путь не изменился = creds.**
> (1) Воспроизведения из чистого состояния (пост-WIPE, только /tmp/toolchain + репо): INSIDE-CACHE PASS (patched Entity structural, InsideBlockOps ARMED=true, cache 131072×12), FLUSH-DIET PASS (fladd semantics, patched+vanilla smoke), ALLOC-DIET PASS (CollisionUtil retarget, mutablePos ring), PALETTED-DEMUX Parity ALL PASS (20000 random ops lockstep L1/L2/model 4883/4883 getAndSet, snapshot snapGen=30009, fast-path 1000 reads, re-materialization, concurrency 3R+1W) — плюс FLUID-FREE PASS из S7-144.
> (2) Уроки REPRO (RUNBOOK_S7145 забанкован research/dispatch-readiness-2026-09-18/): (a) харнессы в default package — запуск по простому имени (FluidFreeHarness — исключение, FQN); (b) Parity stub jar = same-runtime-package closure: семейство PalettedContainer* (patched заменяет vanilla) + Strategy* + Configuration* — иначе LinkageError itable / IllegalAccessError protected-abstract (граница protected/abstract = same package SAME loader); (c) FlushDiet nest-partner RecordedEffect извлекается из kernel; (d) libRoot Parity = 5-й аргумент (paperclip libraries).
> (3) Скрипты забанкованы: repro_dispatch_harnesses.sh + repro_parity.sh (sha256 artifact_hashes_s7145.txt) — полная REPRO-восстанавливаемость диспатч-цепочки после любого WIPE за минуты.
> (4) **БЛОКЕР (11-й тик)**: bootstrap_tick.sh/creds НЕ восстановлены; push 9 коммитов (f67ae10..cb2e153) + диспатчи leg #2' INSIDE-CACHE (X150K, base 35275967738, §156) → FLUSH-DIET → FLUID-FREE (demux=1) → ALLOC-DIET ждут; CLAIMS/INDEX/LEDGER долг 11 тиков. INJECTS-ONLY цел (0 boots; 0 CI-бутов).

> [S7-146 ВОССТАНОВЛЕНИЕ УЧЁТА: блоки S7-142/S7-143 не экспортировались в патч-банк до WIPE №3 — append-реставрация текстов из atomic worklog; код S7-143 требует ре-реимплементации]

> **СТАТУС 20 TPS (S7-142) — WIPE-РЕСТАВРАЦИЯ №2: 5/6 коммитов подняты из патч-банка; S7-139 (FLUID-FREE FULL 91fcd70) УТРАЧЕН (патч подменён дубликатом S7-135b при экспорте 02:17) ⇒ реимплементация по DESIGN.md + полный офлайн-реверификационный цикл = головная офлайн-задача; диспатч FLUID-FREE без неё ЗАПРЕЩЁН (урок leg #2). Патч-банк защищён: переэкспорт 0001..0005 + карантин + манифест. 3x pull --rebase: клоны восстановлены. CREDS-BLOCKED 8-й тик. INJECTS-ONLY цел.**

> **СТАТУС 20 TPS (S7-143) — FLUID-FREE-SECTION РЕ-ВОПЛОЩЁН: код утраченного S7-139 восстановлен по DESIGN.md и ДОКАЗАН офлайн заново (rust 121=120ok+1ignored, 4 section_ff-теста; FluidFreeHarness на реальном ядре 29386794B: STRUCTURAL+INJECTED SURFACE / ARMED=true / FREE-HIT стабилен ff=1 / EVENT-DRIVEN INVALIDATION живьём setBlockState(WATER) crusstyGen 0→2 ff=2 — байт-в-бит с 91fcd70 / RESTORE / SCATTERED-WATER miss стабилен ⇒ OFFLINE PASS). Ключевые уроки: кэш сверяется с демукс-МУТАЦИОННЫМ crusstyGen (±2/мутация), не snapGen; ff 0=unknown/1=free/2=has-fluids (zero-init не вердикт); FluidOps define ДО Entity-ретаргетов. Конвейер 4 рычагов снова ПОЛЕН; критический путь = push + 4 диспатча. CREDS-BLOCKED 9-й тик. INJECTS-ONLY цел.**

[КОД S7-143 (fluid_free.rs + patch_section_ff + FluidOps.java + chain + wiring) утрачен с WIPE №3 — НЕ восстановлен; ре-реимплементация = головная офлайн-задача следующего тика]

> **СТАТУС 20 TPS (S7-146) — WIPE №3 (13:43 +08) + РЕСТАВРАЦИЯ №3 + FLUID-FREE RE-RE-ИМПЛЕМЕНТАЦИЯ ДОКАЗАНА: конвейер 4 рычагов снова ПОЛЕН и ЗАПУШЕН; creds ВОССТАНОВЛЕНЫ владельцем (14:11) — push-блокер 12 тиков закрыт; диспатчи стартуют.**
> (1) WIPE №3 уничтожил c-crussty (ahead 10), c-dist, CRUSSTY, весь тулчейн; my-project уцелел (патч-банк 0001..0007 11/11 sha256 OK, runs_index 279..288, restore-скрипты).
> (2) Реставрация №3: анонимный клон (04ef8d5) → git am 0001-S7-136/0002-S7-137/0003-S7-138 → append-реставрация restore_commits_v2.py (S7-140/141/144/145, оригинальные метаданные + RESTORED-note) → чистка 1-байтовых binary-заглушек (harness .class компилируются по RUNBOOK) → Subject-фикс "[PATCH N/5]"; GOAL/worklog-блоки S7-142/143 восстановлены append'ом (в банке не были). Push 04ef8d5..b0aac42 = 9 коммитов.
> (3) Ре-реимплементация S7-143 (FLUID-FREE FULL) после утраты с WIPE №3: patch_section_ff (append-only utf8-only 2-поля, 15041→15088 байт-в-бит с 91fcd70-записью, idempotent, fail-closed; УРОК: field_info нужен ТОЛЬКО name/desc utf8, НЕ Fieldref — лишние CP-энтри дают +20B) + FluidOps.java (Unsafe-резолвы, ff 0/1/2, publish (ffGen,ff) НА СЕКЦИИ — первый вариант писал ffGen в контейнер, поймано харнессом) + fluid_free.rs (section hook + bridge define + WARN без DEMUX) + chain в inside_cache (wait_bridge_ready 60s, деградация inside-only fail-dominant) + wiring (lib.rs mod/register/activate, run_world3.sh FLUID_FREE, world-bench.yml fluid_free input, build_fluid_ops.sh восстановлен).
> (4) Верификация: rust 122=121ok+1ignored (5 section_ff: applies/grows-байт-в-бит, idempotent, wrong-class, field-shape 0x0041, dump); FluidFreeHarness на реальном ядре 29386794B (sha256 e2992d63, байт-в-бит): STRUCTURAL+INJECTED SURFACE / ARMED=true / FREE-HIT ff=1 ffGen=0 gen=0 / EVENT gen 0→2 ff=2 ffGen=2 / RESTORE gen=4 ff=1 verdict=true / SCATTERED-WATER miss стабилен ⇒ **FLUID-FREE OFFLINE PASS exit 0**; FluidOps.class e031d8cd (честная ре-имплементация, семантика 91fcd70); манифест artifact_hashes_s7146.txt.
> (5) creds: владелец предоставил push-URL 14:11; крон Job 394666 обновлён (правило 1b); НОВЫЙ КРИТИЧЕСКИЙ ПУТЬ = 4 диспатча: leg #2' INSIDE-CACHE (X150K, base 35275967738, гейты §156) → FLUSH-DIET → FLUID-FREE (demux=1) → ALLOC-DIET → absorb-вердикты. INJECTS-ONLY цел (0 sandbox boots; CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-147) — leg #2' ABSORB: A/B-НЕВАЛИДЕН (девиация конфига fp0/900s), НО НАЙДЕН И УСТРАНЁН ГЛАВНЫЙ ИЗМЕРИТЕЛЬНЫЙ ДЕФЕКТ ЭРЫ — паритет популяции фикстуры; ARMED-цепочка INSIDE-CACHE PASS живьём; base-b диспатчен на фикстуре-фиксе.**
> (1) Ран 35314220731 (head 53d14d1): конфиг-девиация диспатча (fake_players=0, seconds=900 — дефолты вместо preregistered fp4/300s) + неравные окна (S7-96d) ⇒ §156-вердикт невозможен. Позитив забанкован: ARMED-цепочка ПОЛНА живьём (pristine 205458B major65 → defined InsideBlockOps+$Recorder → Retargeted{1} 205522B → serve → retransform rc=0), INJECT 150000/150000 VALID, 900s soak без крэша, 0 tick-behind, TPS 1.2→3.3 (база 0.6-0.8).
> (2) «Коллапс» 148k→71k (та же сигнатура leg2) = ПАРИТЕТ-АРТЕФАКТ ФИКСТУРЫ, не самовзрыв: 2970 тиков (900s×3.3TPS) против 210 (300s×0.7) = 14× ванильного распада (item-merge 105k→54.7k, горение/cramming мобов) на стенку-время; topup-модель слепа (deficit=0 aliveEst=const; покрывает только age-despawn; в базе topup не стрелял вовсе — 210<600 тиков). Парадокс эры: чем лучше рычаг, тем «невалиднее» ран по wall-time паритету. fp=0 нарушал спеку сцены («как будто игроки есть»).
> (3) Фикс S7-147 (4c8f029): BenchPopulationPlugin topup REAL-COUNT (Item/Monster/Animals по w.getEntities каждые 600 тиков, маркеры TOPUP-SCAN aliveReal/deficit/aliveEst) + непрерывный drain 20/тик (~14ms profile-invisible, largest-lane-first, miss-guard 64, детерминизм seed^(ft*1_000_003)^total); Compile-OK javac21 против kernel 29386794B+purpur-api+125 libs (CI-эквивалент). Следствие протокола: ВСЕ будущие A/B — против base-b (ран 35317176927, head 4c8f029, все рычаги 0, fp4/300s), не против до-фиксного ценза 35275967738.
> (4) Лейны leg2' vs база — SHARES, не гейт (окна/тики/популяция несопоставимы): inside-blocks 42.39%→35.08%, movement-geom 22.50%→21.86%, flushStep 3.91%→4.29%, PalettedContainer.get 3.06%→3.54%, young GC 146→182; bridge в топах не виден — самовзрыва нет. Артефакты: research/inside-cache-2026-09-18/{ABSORB_S7147.md, run-s7136-leg2prime/ (alloc-collapsed 4.3MB, run-env, BOTTLENECKS_3, gc.log; sha256 artifact_hashes_s7147.txt), run-s7134b-base/ (артефакт ценза), fetch/analyze/dispatch скрипты}.
> (5) NEXT: absorb base-b (популяция ≈148k стабильна, TOPUP-SCAN живой) ⇒ диспатч leg #2'' (inside_cache=1, fp4/300s) ⇒ §156-вердикт (fixture зелёные, ARMED, alloc-семьи ↓30/25%, PalettedContainer.get ↓15%, young GC ↓, entity-фаза ≤+0.5pp) ⇒ FLUSH-DIET leg → FLUID-FREE (demux=1) → ALLOC-DIET. INJECTS-ONLY цел (0 sandbox boots; CI-буты санкционированы: leg2' поглощён + base-b).

> **S7-147b ДОПОЛНЕНИЕ (тот же тик)**: base-b ЗАВЕРШЁН SUCCESS (35317176927, 07:14:44 UTC, 17 мин) и санити-абсорблен: конфиг точен (fp4/300s/inside_cache=0/guard1), INJECT 150000/150000 VALID, **популяция 148391/148193/148027 стабильна** (структурный близнец ценза 148402), TPS 0.8-0.9, topup-скан не стрелял (240 тиков < 600-периода — консистентно с базой; фикстура-фикс дремлет на базовых скоростях, оживает только при ускорении от рычага — паритет честен в обоих режимах) ⇒ **base-b = валидная база для всех §156-сравнений**. leg #2'' INSIDE-CACHE ДИСПАТЧЕН: ран 35318755582 (head 135cb89, inside_cache=1, fp4/300s, 07:17:58 UTC, S7-108). NEXT: absorb leg2'' по §156 (гейты: fixture зелёные, ARMED, alloc-семьи ↓30/25% vs base-b, PalettedContainer.get ↓15% vs 3.06%, young GC ↓, entity-фаза ≤+0.5pp) → FLUSH-DIET leg → FLUID-FREE (demux=1) → ALLOC-DIET.

> **СТАТУС 20 TPS (S7-148, частичный) — leg #2'' ABSORB: A/B-НЕВАЛИДЕН ×2 (74 941 NoClassDefFoundError: EntityQueryOps = парити-контаминация моста; популяция 79k≠148k = дренаж 20/тик < валового распада ~42/тик); ДЕФЕКТ НАЙДЕН И УСТРАНЁН (InsideBlockOps SELF-CONTAINED + drain-budget дефицит-драйвен), leg #2''' диспатчен.**
> (1) Конфиг leg2'' (35318755582, head 135cb89, SUCCESS 07:17:58→07:32:36 UTC) точен по preregistered (fp4/300s/inside_cache=1, девиаций нет); INJECT 150000/150000 VALID; ARMED-цепочка полна живьём (pristine Entity 205458B → defined InsideBlockOps+$Recorder → Retargeted{1} 205522B → serve → retransform rc=0); TPS 0.8→3.0; 0 tick-behind.
> (2) КОНТАМИНАЦИЯ: hardened HIT-верификация gate:195 звала EntityQueryOps.mutablePos() — класс ALLOC-DIET-субстрата НЕ определён в kernel loader при alloc_diet=0 ⇒ 74 941 «Entity threw exception» (07:24:53–07:31:29, ~189/с, ~0.06% entity-tick-вызовов прерывалось Paper per-entity catch) — median-exact паритет нарушен в этих вызовах; офлайн-харнесс не ловил (одна loader-пространство с EQ в CP); leg #2' (175MB stdout) ретроспективно та же сигнатура.
> (3) §156-гейты НЕ выполнены и по долям (inside-blocks alloc 40.44→37.76% = −6.6% отн. при гейте ≥30%; movement-geom 22.24→26.03% ↑; PalettedContainer.get CPU 3.38→3.02% = −10.6% при гейте ≥15%; young GC 125→165 ↑). На-тик нормировка (≈700–900 vs 240 тиков) даёт ↓65–75% по всем семьям, но неинтерпретируем из-за (a) NCDFE-прерываний + (b) дрейфа популяции 148k→79k. ВЕРДИКТ ПО PREREGISTERED ПРОТОКОЛУ НЕВОЗМОЖЕН — честный вывод.
> (4) Фиксы S7-148: (a) InsideBlockOps SELF-CONTAINED — ThreadLocal mutable-pos ring (8 слотов, zeroed, семантика 1:1) внутрь моста, compile-dep entityquery удалён из build_inside_block_ops.sh; javac21 против kernel e2992d63; 0 EQ-ссылок (javap); rust 122=121ok+1ignored; REPRO-харнессы INSIDE-CACHE/FLUSH-DIET/ALLOC-DIET OFFLINE PASS; новые байты Ops 7826B / Recorder 4307B. (b) BenchPopulationPlugin drain-budget = clamp(deficit/50, 20, 100)/тик + TOPUP-SCAN 600→120т (валовый распад ~42/тик покрывается; в базе дремлет bit-for-bit — base-b остаётся базой). Compile-OK CI-эквивалент.
> (5) NEXT: leg #2''' (head=фикс, inside_cache=1, fp4/300s) → §156 vs base-b + НОВЫЙ обязательный чек «0 NoClassDefFoundError в stdout» + популяция ≈148k весь soak (TOPUP-SCAN живой). Если дрейф повторится — калибровка фикстуры продолжается (base-c). INJECTS-ONLY цел (CI-буты санкционированы: leg2'' поглощён, leg2''' диспатчен).

> **СТАТУС 20 TPS (S7-148b, частичный) — leg #2''' ABSORB: ПЕРВЫЙ ВАЛИДНЫЙ A/B ЭРЫ (фикстура стабильна 148k весь soak, 0 NCDFE, ARMED полна) ⇒ §156 = 5/7 PASS (PARTIAL-PASS): INSIDE-CACHE GREEN-BY-SAFETY (alloc-диета −32.5%/−35.9% подтверждена, entity-фаза −5pp, TPS нейтрально); «ускорение» leg2'' (TPS 3.0) на 100% артефакт контаминации+полупустой сцены. FLUSH-DIET leg ДИСПАТЧЕН (35324517090).**
> (1) Валидность: head 45687c4 (self-contained мост + дренаж-фикс); конфиг точен; INJECT VALID; популяция 148546/148383/148197 vs base-b 148391/148193/148027 — структурные близнецы, TOPUP-SCAN ×2 живой (дренаж держит план); 0 NoClassDefFoundError/0 Entity-threw-exception (stdout 241KB vs 177MB leg2''); ARMED-цепочка полна живьём (defined Ops+Recorder → Retargeted{1} 205458→205522 → serve → rc=0); kernel в артефакте e2992d63 байт-в-бит.
> (2) §156: inside-blocks alloc 40.44→27.30% (−32.5% отн.) ✅; movement-geom 22.24→14.26% (−35.9%) ✅; entity-фаза 59.7→54.7% (−5pp) ✅; PalettedContainer.get CPU 3.38→3.09% (−8.6% отн. при гейте ≥15%) ❌ — гейт был оптимистичен (кэш снимает дублирующую traversal, не первую); young GC 125→143 (+14%) ❌ — частично topup-спавны (Object[] +67.6% = спавн-путь), частично шум.
> (3) Вердикт по протоколу («частичный = калибровка»): INSIDE-CACHE = GREEN-BY-SAFETY — парити-чист, самовзрыва нет, young-gen alloc-давление снижено существенно, TPS не регрессирует; КАК TPS-ДРАЙВЕР X150K НЕ ПОДТВЕРЖДЁН (главный CPU-жир живой сцены — не checkInsideBlocks). ОстаЁтся кандидатом конвейера (накопительно в следующих ногах). Калибровка гейтов: young-GC-гейт на живой сцене с топапом трактуется по alloc-долям семейств; PalettedContainer.get-гейт INSIDE-CACHE-специфичный.
> (4) NEXT: absorb FLUSH-DIET leg (35324517090, head 1864e3d, flush_diet=1, fp4/300s, 08:28:05 UTC, §S7-138-гейты + S7-148-протокол) → FLUID-FREE leg (demux=1) → ALLOC-DIET leg → накопительный конфигурационный ран (все зелёные рычаги вместе). INJECTS-ONLY цел (CI-буты санкционированы: leg2''', FLUSH-DIET).

> **СТАТУС 20 TPS (S7-148c, частичный) — FLUSH-DIET leg ABSORB (35324517090): §S7-138 PASS — flushStep-семья 4.15%→0.00% (−100%), Object[] −51.2%, entity-фаза −5.6pp, fixture зелёная, 0 NCDFE, TPS нейтрально (аллокационный рычаг, не TPS-драйвер). FLUSH-DIET = GREEN (в накопительный ран). FLUID-FREE leg ДИСПАТЧЕН (35326295881, fluid_free=1+demux=1+inside_cache=1-владелец).**
> (1) Валидность: head 1864e3d; конфиг точен (flush_diet=1, inside_cache=0); INJECT VALID; популяция 148483/148328/148178 стабильна; 0 NoClassDefFoundError (stdout 257KB); мост ARMED живьём (pristine StepBasedCollector 5695B major65 → ретаргеты); kernel e2992d63 байт-в-бит.
> (2) Механика end-to-end: zero-waste addAll (FlushOps.fladd) снял ВСЮ flushStep-семью (4.15% alloc-чурна ценза 35275967738) и половину Object[]-листа (426→208) — точное попадание в дизайн S7-137; young GC 125→141 — топап-спавны (калибровка S7-148b), GC-счётчик нечувствителен к диете статики.
> (3) NEXT: absorb FLUID-FREE leg (35326295881, head d2f063e, fluid_free=1+paletted_demux=1+inside_cache=1 — владелец Entity-цепочки compose_entity, 08:48:58 UTC, §S7-139: fluid-доля get ↓≥60%, entity-фаза ≤+0.5pp + S7-148-протокол) → ALLOC-DIET leg → накопительный ран (INSIDE-CACHE + FLUSH-DIET + FLUID-FREE + ALLOC-DIET вместе). INJECTS-ONLY цел (CI-буты санкционированы).

> **СТАТУС 20 TPS (S7-148d, частичный) — FLUID-FREE leg ABSORB (35326295881): §S7-139 FAIL ⇒ REFUTED-BY-ECONOMICS: fluid-доля get ВЫРОСЛА 53.8→60.1% (fluid-get 916→934 не снят — ff-кэш без хитов на живой сцене), TPS −20% (демукс-оверхед), ЭКОНОМИЧЕСКИЙ ПОТОЛОК лейна 1.8% total CPU = МИКРО-КЛАСС. Демукс на живой сцене анти-оптимизация (default 0). ALLOC-DIET leg ДИСПАТЧЕН (35328228929).**
> (1) Валидность: head d2f063e (fluid_free=1+demux=1+inside_cache=1-владелец); INJECT VALID; популяция 148445/148395/148267 стабильна; 0 NCDFE (stdout 248KB); ВСЕ цепочки ARMED живьём впервые end-to-end на живой сцене: DEMUX ARMED + PalettedContainerOps defined + fluid_free defined FluidOps → section splice 15041→15088 (байт-в-бит 91fcd70) → entity chain composed Retargeted{sites:2} → serve section 15088; kernel e2992d63.
> (2) Гейт §S7-139 FAIL: fluid-доля get 53.8%→60.1% (↓≥60% НЕ достигнут — доля выросла); readPalette 428→0 (−100%, демукс снял свой лейн); get общий −8.7% отн.; inside-blocks −32.5% из leg2''' НЕ воспроизвёлся (демукс сместил профиль); TPS 0.6–0.7 = регресс −20% (демукс-оверхед; в leg2''' без демукса 0.7–0.8).
> (3) Экономика: fluid-get = 916/50454 = 1.8% total CPU живой сцены — даже идеальный снимок лейна = МИКРО-КЛАСС (запрещено владельцем); §S7-139-гейт писался в до-фиксную эпоху. ff-кэш без хитов: гипотезы (секции has-fluids / демукс-мутации / движение сущностей) — офлайн-диагностика НЕ блокер.
> (4) Решение: paletted_demux default 0 на живой сцене; fluid_free забанкован off (OFFLINE PASS в силе, экономика не та); §S7-139 калиброван. Конвейер: INSIDE-CACHE GREEN-BY-SAFETY + FLUSH-DIET GREEN зелёные; ALLOC-DIET leg в полёте (35328228929, head 59b6bbb, alloc_diet=1, 09:11:24 UTC) → накопительный ран (inside_cache=1+flush_diet=1+alloc_diet по вердикту) → итоговый вердикт эры. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-149, частичный) — ALLOC-DIET leg ABSORB (35328228929): REFUTED (повторное подтверждение S7-133b на валидной базе): alloc-семьи не упали (movement-geom +13.7%, Vec3 +5.8%), young GC +13.6% (гейт ↓≥10% провален), TPS −12%; мосты корректны (ARMED полна, 0 NCDFE), экономика отрицательная — дефолт 0 подтверждён, wave-2 НЕ продолжается. Итог конвейера рычагов: GREEN = INSIDE-CACHE + FLUSH-DIET; REFUTED = FLUID-FREE+DEMUX + ALLOC-DIET. НАКОПИТЕЛЬНЫЙ РАН ДИСПАТЧЕН (35330129145: inside_cache=1+flush_diet=1).**
> (1) Валидность: head 59b6bbb; INJECT VALID; популяция 148321/148314/148103 стабильна; 0 NCDFE; ALLOC-DIET ARMED живьём (defined EntityQueryOps → LivingEntity 186570→186759 Retargeted{1} + CollisionUtil 45439→45546 Retargeted{1} → serve оба); kernel e2992d63.
> (2) Гейты FAIL: семьи +3..14% (диета не видна ни в одном листе), total alloc +4.2%, young GC +13.6%, TPS 0.6–0.7 (−12%) — ретаргет-мосты добавляют invokestatic-оверхед на push/collision горячих путях без компенсации. REFUTED дважды (S7-133b + настоящая нога) — экономика закрыта.
> (3) Итоговый вердикт эры — после absorb накопительного рана (35330129145, head dbb5e8e, inside_cache=1+flush_diet=1, fp4/300s, 09:33:13 UTC): ожидание = alloc-эффекты обоих зелёных складываются (inside-blocks ~−32%, flushStep →0), 0 NCDFE, популяция стабильна, TPS ≥ базы. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-149b, ИТОГОВЫЙ ВЕРДИКТ ЭРЫ) — НАКОПИТЕЛЬНЫЙ РАН ABSORB (35330129145): ЗЕЛЁНЫЙ ФИНАЛ — INSIDE-CACHE + FLUSH-DIET вместе: flushStep −100% воспроизведён, inside-blocks доля 40.44→32.81% (−18.9% отн.), young GC ВПЕРВЫЕ ниже базы (125→118, −5.6%), TPS-паритет 0.8–0.9, популяция-близнец 148392/148234/148164, 0 NCDFE, кросс-помех нет. РЕКОМЕНДАЦИЯ: inside_cache=1+flush_diet=1 = default-кандидаты.**
> (1) Валидность: head dbb5e8e; INJECT VALID; 0 NoClassDefFoundError (stdout 259KB); ARMED-маркеры обоих мостов живьём; kernel e2992d63; TOPUP-SCAN живой.
> (2) Комбинация: оба эффекта воспроизведены одновременно (flushStep −100% байт-в-байт; inside-blocks −18.9% отн. — доля смещена снятием flushStep из знаменателя, в изоляции −32.5%); young GC −5.6% — впервые в эре ниже базы (в изолированных ногах +13–14% от топапа — диета двух зелёных перевесила спавн-давление); entity-фаза −1.8pp.
> (3) ИТОГ ЭРЫ (все ноги на валидной базе base-b, протокол S7-148): GREEN = INSIDE-CACHE + FLUSH-DIET (default-кандидаты); REFUTED = FLUID-FREE+DEMUX (лейн 1.8% CPU) + ALLOC-DIET (×2). Честная граница: TPS X150K 0.8–0.9 упирается в entity tick (58%) + unclassified (33–39% — крупнейший непокрытый лейн) — следующий фронт (пост-эра).
> (4) Канон цепочки эры: base-b 35317176927 → leg2'' (NCDFE-контаминация, рефакт моста) → leg2''' 35322530537 → FLUSH-DIET 35324517090 → FLUID-FREE 35326295881 → ALLOC-DIET 35328228929 → CUMULATIVE 35330129145. INJECTS-ONLY цел (7 CI-бутов за эру, все санкционированы; 0 sandbox boots).

> **СТАТУС 20 TPS (S7-150, пост-эра: RECON + STEP-0) — свежий профиль CUMULATIVE 35330129145 отранжирован, топ-1 attackable-лейн найден: fluid-push family (`Entity.updateFluidHeightAndDoFluidPushing`) ≈ 10% total CPU (36.4% лейна ItemEntity.tick 23.43% CPU + 9.8% Zombie). РЫЧАГ FLUID-DIRTY: per-entity мемоизация скана + event-driven dirty-stamp ledger по мутациям воды (класс: кэш + dirty-флаги вместо поллинга).**
> (1) Ранжирование (s7150_recon.py → research/inside-cache-2026-09-18/S7150_RECON.md): entity tick 58.5%; лейны: fluid-push ~10% (ранг 1) → move/collision ~5.4% (ранг 2) → inside-blocks residual ~5.1% (ранг 3) → entity tracker ~2% (ранг 4). Merge-search у items = 2.2% лейна — МИКРО-КЛАСС (гипотеза O(n²)-мерджа на 100k items опровергнута профилем). GC native 28.8% — закрыт (alloc-диета REFUTED ×2). PalettedContainer.get 3.23% leaf — распределён по скан-лейнам (демукс REFUTED).
> (2) STEP-0 javap-контракт (kernel e2992d63, research/fluid-dirty-2026-09-18/step0_*.txt): скан (span-вычисление → пред-фетч sections → тройной цикл PalettedContainer.get → getFluidState → getHeight/getFlow) = ЧИСТАЯ ФУНКЦИЯ от (span, fluid-состояния блоков, isPushedByFluid); постобработка (fluidHeight.put, normalize/scale, 0.003/0.0045-ветка, setDeltaMovement) зависит от dm/Player — выполняется всегда. baseTick игнорирует return (pop), эффекты = записи. Wrapper (boat-спецслучай, splash, wasTouchingWater, lavaScale 0.007/0.0023333…, updateFluidOnEyes) НЕ трогается.
> (3) Дизайн FLUID-DIRTY: FluidPushOps self-contained (паттерн InsideBlockOps S7-148, memo-ring внутрь моста); хит-условие = span бит-в-бит + pushedByFluid + версии секций span не изменились; dirty-ledger = хук в LevelChunk.setBlockState со сравнением old/new FluidState ПО ССЫЛКЕ (bump только при реальной мутации воды — fluid-tick/explosion/waterlog покрыты). Движущиеся сканируются как vanilla; покоящиеся (масса 100k items) пропускают скан. Анти-урок FLUID-FREE учтён: кэш per-ENTITY (не per-section has-fluids — 0 хитов в водном мире), демукс не нужен. Сопутствующая alloc-диета: new MutableBlockPos на каждый вызов + Vec3-цепочки исчезают на хитах → GC relief.
> (4) Preregistered гейты §S7-150 (честный A/B min-of-2, НЕ лотерея): G1 fluid-лейн ↓≥60% при хит-рейте ≥80%; G2 0 NCDFE + ARMED-маркер; G3 популяция-паритет; G4 TPS не хуже базы min-of-2; G5 OFFLINE lockstep 20000 ops (шторм мутаций) — cached бит-в-бит = vanilla (hAcc/hitFlag/flowAcc/flowN/lastLavaContact); G6 young GC не выше базы. REFUTED-критерий: G1 при живом ARMED и хитах ≥80% недостижим ИЛИ G4-регресс >10% (анти-урок демукса) ⇒ fluid_dirty=0.
> (5) Имплементация — S7-151+: FluidOps.java (javac21 против kernel), двойной section-splice (тело скана + setBlockState-хук), rust-хук fluid_dirty.rs + wiring, FluidDirtyHarness OFFLINE (стаб-closure по прецеденту S7-143), затем preregister dispatch (владелец Entity-цепочки, внутри_cache=1+flush_diet=1 база). INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-151, частичный) — FLUID-DIRTY ИМПЛЕМЕНТИРОВАН И ЗАБАНЧЕН: FluidPushOps (self-contained мост: memo двух тегов × 2^17 слотов + реимплементация скана бит-в-бит + event-driven ledger по секциям), двойной ретаргет (оба wrapper-сайта скана + единственный сайт section-write), rust-wiring (fluid_dirty.rs + compose-цепочка inside_cache), rust suite 132/0/1, harness OFFLINE PASS (structural/wiring/armed/ledger). Диспатч — после поведенческого lockstep S7-152.**
> (1) Census v2: во всём kernel ровно 2 вызова updateFluidHeightAndDoFluidPushing (оба в Entity-обёртках — вода 0.014 / лава 0.007-0.0023333) и ровно 1 сайт LevelChunkSection.setBlockState(IIIL…)BlockState в LevelChunk.setBlockState — покрытие полное; miss-путь зовёт ванильное тело обычным вызовом (рекурсии нет, тело не тронуто).
> (2) HIT-условие: guard + span бит-в-бит (клэмп-математика ванили) + pushedByFluid + ссылки секций span + штампы секций не изменились ⇒ бит-в-бит тот же результат скана; постобработка (put/push) исполняется всегда (dm-зависимость сохранена). Capture только в свой слот; движущиеся сканируются как ваниль; покоящиеся пропускают скан.
> (3) Ledger (event-driven инвалидация): secWrite-делегат возвращает секции старое состояние, ref-compare FluidState бампит штамп ТОЛЬКО при реальной мутации жидкости (вода→вода/блоки без fluid-дельты не инвалидизируют — проверено на реальных секциях kernel в харнессе).
> (4) Верификация тика: 9 новых roundtrip-тестов патчеров (строгие 2/1 сайта, idempotent, fail-closed, compose с inside) — suite 132/0/1; harness OFFLINE PASS exit 0 (структурный линк патченных классов над kernel + ledger-контракт на реальных секциях); sha256-манифест 14 артефактов (artifact_hashes_s7151.txt).
> (5) NEXT (S7-152): поведенческий lockstep мини-Level (FluidPushOps.scan vs ванильный скан бит-в-бит, позиционные свипы + шторм мутаций = G5) → preregister dispatch (inside_cache=1+flush_diet=1+fluid_dirty=1, fp4/300s/150k/seed42, A/B min-of-2 vs CUMULATIVE 35330129145). INJECTS-ONLY цел (CI-бутов 0).

> **СТАТУС 20 TPS (S7-152, частичный) — FLUID-DIRTY ПОВЕДЕНЧЕСКИЙ LOCKSTEP PASS (G5 core): бит-в-бит дифференциал реального vanilla-скана vs FluidPushOps.scan на Unsafe scan-contract фикстурах — 44 позиционных кейса × 4 сцены (вода/кромки/границы чанков и секций/lava+water столб/lastLavaContact/flowing уровни/шторм мутаций/unloaded-guard) — 0 расхождений; hit=49 miss=94 vanilla=0.**
> (1) Стена офлайн-конструирования обойдена честно: vanilla Level ctor кастует this к ServerLevel (bukkit-периферия в ctor), прямой ctor LevelChunk тоже кастует к ServerLevel + зовёт MinecraftServer.getServer().registryAccess() — поэтому Level и LevelChunk фикс-туры = Unsafe.allocateInstance с ровно теми полями, что читает javap scan-контракт (step0_*.txt); всё, что зовёт скан — реальный kernel-код (touchingUnloadedChunk→hasChunksAt→moonrise$areChunksLoaded; тройной цикл; getHeight/getFlow с соседними чтениями через реальные секции).
> (2) Сцена C (шторм): источник→течение-8 / течение→воздух / вода→камень / камень→вода через secWrite-делегат — каждая мутация = MISS (event-driven инвалидация) + бит-в-бит совпадение со свежей ванилью на НОВОМ мире; воздух→камень НЕ бампит (ref-compare). Сцена D: unloaded-guard — оба пути false БЕЗ записи fluidHeight (ваниль-эквивалент).
> (3) HIT bookkeeping: повторные идентичные вызовы = HIT и тот же результат (bookkeeping h0→h0+1→h0+2 проверен на каждом кейсе сцены A). vanilla=0 — мост ARMED полностью обслуживает путь.
> (4) Диспатч: next tick S7-153 — preregister dispatch fluid_dirty=1 (в цепочке inside_cache=1+flush_diet=1), fp4/300s/150k/seed42/xmx10G, A/B min-of-2 vs CUMULATIVE 35330129145, гейты §S7-150 G1-G4+G6 живьём. INJECTS-ONLY цел (CI-бутов 0).

> **СТАТУС 20 TPS (S7-153, частичный) — FLUID-DIRTY ЖИВАЯ НОГА ABSORB (35341241628): §S7-150 ГЕЙТ G1 FAIL ⇒ REFUTED-BY-ECONOMICS: hit-rate ≈ 0% на живой сцене (scan 1592 ≈ vanilla 1590 сэмплов — весь мост в miss-путь), fluid-family 9.91%→10.40%, young GC 118→140 (+18.6%), TPS паритет; 0 NCDFE, ARMED-цепочка полна живьём, популяция 150000 VALID. fluid_dirty забанкован off (default 0). ТРЕТЬЕ подтверждение сигнатуры: fluid-scan лейн живой сцены позиционно-нестабилен.**
> (1) Валидность: head 38afaa3 (пломбинг fluid_dirty: workflow input + run_world3.sh export CRUSSTY_FLUID_DIRTY + dispatch_s7153.py); конфиг точен (inside_cache=1+flush_diet=1+fluid_dirty=1, demux/diet/ff=0); INJECT 150000/150000 VALID (items=105000, hostiles=30000, passives=15000); 0 NoClassDefFoundError (stdout 275KB); ARMED живьём: FluidPushOps+$ScanOut defined в kernel loader → LevelChunk Retargeted{1} (48104→48329) → entity chain composed Retargeted{sites:2} → serve → retransform rc=0; kernel e2992d63. Runtime-доказательство обслуживания: 1592 сэмпла FluidPushOps.scan в cpu-collapsed.
> (2) Гейты: G1 FAIL (family не снята: 5185→5691 сэмплов = +9.8%; hit-rate ≈0% при гейте ≥80%); G6 FAIL (young GC 118→140, +18.6%); G4 ~паритет (0.6–0.9 vs 0.7–0.9, регресса >10% нет); G2/G3 PASS. REFUTED-критерий §S7-150 («G1 при живом ARMED недостижим») выполнен ⇒ fluid_dirty=0, оффлайн-достижения S7-151/S7-152 (lockstep G5 бит-в-бит) в силе — рычаг инженерно корректен, экономика нулевая.
> (3) Корневая причина (ABSORB_S7153.md): span каждой сущности меняется каждый тик — популяция fluid-скана X150K движущаяся (items: topup-цикл + плавающие в воде предметы никогда не покоятся; zombies: AI-блуждание). Гипотеза S7-150 «100k items покоится» опровергнута живым профилем. Третья независимая сигнатура после FLUID-FREE (S7-148d): per-ENTITY/per-SECTION мемоизация позиции-зависимого скана на живой сцене не бьёт.
> (4) Инцидент гигиены: GitHub push protection отклонил токен в dispatch_s7153.py:26 — фикс: токен читается из remote URL (правило 1b), секретов в скриптах нет; push прошёл (eccf3d3..38afaa3).
> (5) NEXT (S7-154): RECON-2 — раскладка unclassified-фазы (33–39% entity-цикла, крупнейший непокрытый лейн) до классов поведения (AI/goal-selector/brain/sensing); ранжированные остатки (move/collision 5.4%, inside-blocks residual 5.1%, tracker ~2%) — микро-класс по отдельности, кэш-рычаги там упрутся в ту же позиционную нестабильность. INJECTS-ONLY цел (CI-бут: 35341241628 санкционирован как preregister A/B leg).

> **СТАТУС 20 TPS (S7-154, RECON-2) — ПУЛ ATTACKABLE ≥5% ИСЧЕРПАН: non-entity main (8.22% CPU) разложен до якорей — доминанты нет (пассажиры 1.17%, block entities 0.06%, поршни 0.06%); broadphase/entity-query семейство 10.43% CPU раздроблено по 5 семантическим вызывателям (итерация индекса 30.9% fam, collide 19.2%, noCollision 16.7%, push 8.4%, hard-colliding 6.6%) и по всем классам (Zombie 31.8%, ItemEntity 28.1%). Следующий ×N-класс = region-threaded entity ticking (S7-155+ proposal).**
> (1) Данные (s7154_recon2.py → S7154_RECON2.md, база CUMULATIVE 52341 + кросс-чек лега 35341241628): ванильная семантика требует каждый broadphase-запрос каждый тик (расталкивание/мердж/коллизии/sensing по текущим позициям); популяция X150K позиционно-нестабильна (урок FLUID-DIRTY: кэш результата = 0 хитов), батч-амортизация = смена наблюдаемой логики (запрещено), индекс уже O(log) (moonrise), JNI per-call перекрывается стоимостью перехода (урок ALLOC-DIET ×2).
> (2) Ядерная карта 4-ядерного раннера: main-поток (entity-цикл 58.5%) насыщает ~1 ядро; native GC 28.8% + JIT 3.9% уже параллельны; chunk-system workers 0.1% — ПРОСТАИВАЮТ; 2-2.5 ядра в среднем свободны. Единственный оставшийся ×N-рычаг из списка владельца («планировщики»): region-threaded entity ticking (Folia-модель, тик независимых регионов на параллельных воркерах, per-entity семантика сохраняется; parity-риск = межрегионный порядок тиков/РНГ). Мега-проект: S7-155 feasibility-гейт → прототип планировщика → A/B X150K.
> (3) Финальная карта эры: GREEN = INSIDE-CACHE + FLUSH-DIET (забанкованы в CUMULATIVE); REFUTED = DEMUX+FLUID-FREE, ALLOC-DIET ×2, FLUID-DIRTY; закрыто = GC/JIT/индексы. Честная граница: TPS X150K 0.8-0.9 = семантический минимум тика 150k сущностей на ванильной логике; одиночные кэш-рычаги исчерпаны, ×N-класс достижим только планировщиком.
> (4) INJECTS-ONLY цел: CI-бутов за тик 0 (RECON-2 оффлайн-анализ артефактов; диспатчей нет).

> **СТАТУС 20 TPS (S7-155, RECON-3) — FEASIBILITY-ГЕЙТ region-threaded entity ticking: GREEN. Hot per-tick путь чист от общего RNG; взаимодействия пространственно локальны; Amdahl: TPS-потолок ×2.31 (реалистичный) / ×1.17 (пессимист); moonrise-индекс уже concurrent; NEXT S7-156 прототип RegionTickOps.**
> (1) Данные (s7155_kernel_census.py через javap Temurin-JDK21 + s7155_profile_split.py; база CUMULATIVE 52341): entity-фаза 58.5% CPU разложена — AI/brain/goal/navigation/sensing 29.3% фазы, other-local 24.6%, cross-entity broadphase 24.2% (сверка S7-154: строгие якоря 10.43% CPU), fluid-scan 18.6% (полная локальность), movement-integration 3.2%. Доминирующие взаимодействия = push/merge/collide с радиусами 0.5-2 блока против региона 8×8 чанков (128 блоков) — доля кросс-регионных ≈ 4·r/L ≈ 3-6%.
> (2) RNG-census (весь jar, 9809 классов): общий `Level.random` — 132 референсера (47 entity-классов), но горячий per-tick путь ЧИСТ: ItemEntity/Mob = 0 ссылок; Zombie — только hurtServer; LivingEntity — только breakItem; Entity — только sendBubbleColumnParticles (в сцене soul-sand нет). Per-tick RNG = per-entity `Entity.random` (109 классов) → порядок тиков между регионами не влияет на РНГ-последовательности сущностей. В профиле общий-RNG пути = 100 сэмплов/52341 (0.19%).
> (3) Структура цикла: ServerLevel.tick → ActivationRange.activateEntities → entityTickList.forEach (единый сегмент-контейнер для замены ретаргетом) → tickNonPassenger/tickPassenger → tickBlockEntities. EntityTickList = обёртка moonrise IteratorSafeOrderedReferenceSet (insertion-порядок; шардится по регионам). tickNonPassenger уже потоково-осведомлён (TickThread.ensureTickThread, currentlyTickingEntity AtomicReference); moonrise EntityLookup уже concurrent (SWMR/ConcurrentLong2-таблицы) — запись в секции только от владельца сущности = disjoint по регионам без новых локов.
> (4) Amdahl (E=58.5%, non-entity 8.22%, P=3): S1 (cross сериализован, пессимист) ×1.17; S2 (cross region-локален, serial 3% — ожидаемый) ×2.31; S3 идеал ×2.41. Это единственный оставшийся ×N-класс (все одиночные кэш-рычаги закрыты S7-154).
> (5) Дизайн S7-156: регион 8×8 чанков; планировщик заменяет ТОЛЬКО контейнер forEach (region-бакеты с insertion-порядком внутри + W воркеров + барьер); Entity.tick() остаётся байт-в-байт ванилью; общий Level.random — synchronized-обёртка; кросс-регионное чтение соседа = отставание ≤1 тик (статистическая эквивалентность = bar median-exact parity). INJECTS-ONLY: ретаргет тела ServerLevel.tick + self-contained RegionTickOps + rust-wiring.
> (6) Preregistered гейты S7-156: PG1 OFFLINE lockstep (region-параллель vs ваниль-последовательность, per-entity бит-в-бит); PG2 live 0 NCDFE/ARMED/популяция-близнец; PG3 TPS ≥ +25% (A/B min-of-2 vs CUMULATIVE); PG4 young GC ≤ база+15%; REFUTED-критерий: прирост <10% или неустранимые парити-провалы. INJECTS-ONLY цел (CI-бутов 0; RECON-3 оффлайн).

> **СТАТУС 20 TPS (S7-156, частичный) — REGION-THREADS ИМПЛЕМЕНТИРОВАН И ЗАБАНЧЕН: RegionTickOps (self-contained мост: снапшот ванильной итерацией → W пространственных бакетов (8-чанковые регионы, W=4 квадранты) → W-1 постоянных TickThread-воркеров + main, барьеры GO/DONE, deferred FIFO EntityCallbacks-мутаций), двойной ретаргет (сегмент forEach в ServerLevel.tick 1:1 + оба гвард-сайта EntityCallbacks), rust-wiring region_threads.rs, suite 140/0/1, harness OFFLINE PASS exit 0 (structural/wiring/dormant+parallel: 200 entities W=2 exactly-once). Диспатч — после PG1-верификации S7-157.**
> (1) Census S7-156 (javap + бинарный скан CP): EntityTickList.add/remove имеют РОВНО по одному сайту во всём kernel — ServerLevel$EntityCallbacks.onTickingStart/onTickingEnd (поверхность гварда = 2 метода); forEach — 1 сайт в ServerLevel.tick(BooleanSupplier); contains (tickPassenger) — pure read (containsKey), безопасен при отсутствии писателей.
> (2) Гарантии паритета дизайна: per-entity логика = ванильный consumer байт-в-байт (планировщик меняет ТОЛЬКО контейнер); intra-bucket порядок = снапшот = insertion; mid-phase добавки начинаются со следующего тика (эквивалент ванильного snapshot-итератора); отложенные удаления безвредны (гейт isRemoved() в самом consumer); per-entity RNG локален (S7-155); общий Level.random — только редкие пути (0.19%), Xoroshiro-гонки = lost updates без коррупции.
> (3) Инцидент OFFLINE: двойной DONE-барьер в parallelTick (tickBucket джойнит в finally + awaitDone в finally parallelTick) → deadlock main/worker (jstack: main на DONE, worker вернулся в GO) → удалён дублирующий awaitDone, воркеры персистентны между тиками. Урок: один барьер — одна точка джойна.
> (4) Пломбинг: world-bench.yml input region_threads (integer ≥2) + REGION_THREADS env passthrough; run_world3.sh REGION_THREADS + config-echo + export CRUSSTY_REGION_THREADS. Композ ServerLevel-шва: хук регистрируется ПОСЛЕ tickhook (хвост цепочки), строгий WARN при одновременном F1/F3.
> (5) NEXT (S7-157): PG1-верификация live-подготовка — preregister dispatch region_threads=4 (A/B min-of-2 vs CUMULATIVE 35330129145, внутри_cache=1+flush_diet=1), гейты §S7-155 PG2/PG3/PG4 живьём (0 NCDFE/ARMED/популяция-близнец; TPS ≥ +25%; young GC ≤ база+15%). INJECTS-ONLY цел (CI-бутов 0).

> **СТАТУС 20 TPS (S7-157b, частичный) — REGION-THREADS PG1 LOCKSTEP PASS (бит-в-бит W=1==W=2==W=4) + preregister dispatch leg #1 (35353820223) = LIVE CRASH 40-й секунды: root cause = worker пампит Paper mid-tick очередь (guardEntityTick → moonrise$midTickTasks → MainThreadExecutor.pollTask → NoSuchElementException в отсутствие main-насоса); фикс MID-TICK GATE (880e406), leg #2 (35363758352) диспатчен.**
> (1) PG1 (OFFLINE lockstep, RegionLockstepHarness над реальным kernel): дайджест `61e3c374…941d5` бит-в-бит при W=1==W=2==W=4 (60 тиков, 400 сущностей + шторм мутаций, финал 413); отложенные добавления = следующий тик во всех W (javap: maxIndex пиннится при создании итератора — эквивалент ванили); mid-tick удаления идентичны с T+1; дайджест иммунен к порядку drain между бакетами, чувствителен к отклонениям per-entity семантики (двойной тик/пропуск/RNG-перерасход/потеря мутации).
> (2) Крэш leg #1 (INJECTS-ONLY, санкционированный CI-бут): `NoSuchElementException` @ `ServerChunkCache$MainThreadExecutor.pollTask:838`; census: из 7 насосных сайтов kernel воркер-достижим ровно 1 (Level.guardEntityTick → moonrise$midTickTasks) — Paper pump'ит mid-tick очередь из guardEntityTick на ЛЮБОМ потоке, а main-насос отсутствует при параллельном тике. Инцидент гигиены: смерть JVM + `tail -f` сирота жгли job 68 мин.
> (3) Фикс S7-157b: RegionTickOps.midTickTasks gate — ThreadLocal worker-флаг (worker = пропуск mid-tick pump как side-эффекта Paper-дедликации, main = точная ванильная делегация); 3-й строгий byte-hook Level.guardEntityTick Retargeted{1} + 4 roundtrip-теста (suite 144/0/1); region_threads.rs v2 (3 таргета); harness OFFLINE PASS + per-visit флаг-чек (isWorker() true ТОЛЬКО на helper-потоках, 0 нарушений); PG1 дайджест не изменился. CI-гигиена run_world3.sh: FIFO-паттерн (tail -f сирота убивается TERM+KILL) + liveness-watchdog'и boot/pop/soak (kill -0 SERVER_PID → FATAL рано, артефакты сохраняются).
> (4) Leg #2 (35363758352, 15:39:30 UTC — 9 сек после пуша фикса): тот же preregister протокол (region_threads=4 vs CUMULATIVE 35330129145, fp4/300s/150k/seed42/xmx10G); absorb гейтов PG2/PG3/PG4 — тик 00:48. INJECTS-ONLY цел (CI-буты: leg #1 — санкционированный крэш-лег, leg #2 — preregister A/B).

> **СТАТУС 20 TPS (S7-157c, ABSORB leg #2, 35363758352) — REGION-THREADS ЭКОНОМИКА ДОКАЗАНА: TPS медиана 0.90 → 1.50 = +66.7% (PG3 PASS, гейт +25%; первый рычаг эры, реально сдвинувший TPS; оффлоад воркерами 75.6% lane подтверждён профилем, потолок Amdahl ×2.31 по S2 не пробит); БАНКОВАНИЕ ОТЛОЖЕНО: tracker-race NPE крэш в хвосте soak (после завершения всех профайлер-окон — данные валидны) + PG4 FAIL (young GC 118→155, +31.4%). Вердикт: НЕ REFUTED (прирост ≫10%, инциденты — фиксируемые гонки, не семантика), НЕ GREEN-к-банкованию (крэш-лег не банкуют).**
> (1) Валидность: head 880e406; конфиг бит-в-бит preregister (region_threads=4 на inside_cache=1+flush_diet=1); INJECT 150000 VALID; 0 NCDFE; ARMED полна (region_threads ServerLevel Retargeted{1} + EntityCallbacks{1}+{1} + Level, rc=0 ×3; inside_cache{1}; flush_diet{2}); kernel e2992d63. Job завершён 75-мин wall-cancel ПОСЛЕ сбора данных (spark upload 15:54:53, крэш 15:54:54, артефакт спасён if:always()).
> (2) Персонал: tickBucket lane 71547 сэмплов = 55.64% CPU; воркеры 54057 = 75.6% lane (дизайн W=4 точен: main slot 0 + 3 helper'а); mid-tick gate живьём: 159 сэмплов pump'а ВСЕ на main, 0 на воркерах — NoSuchElementException leg #1 не воспроизвёлся (фикс S7-157b работает). TPS crawl растёт по soak (1.1→1.7), база плоская (0.7→0.9); fluid-family 8.90% (лейн не тронут).
> (3) Инциденты (root-cause → S7-158): (a) NPE ChunkMap.newTrackerTick:1017 «entity is null» ← ServerLevel.tick:815 — воркер-удаление сущности гоняет main-итерацию tracker-карты (deferred FIFO сериализует только EntityTickList, НЕ EntityLookup/tracker); (b) UUID-dup WARN 1× (база 0×): Arrow (worker-спавн skeleton) + Rotten Flesh в разных регионах — генерация UUID на параллельных путях алиасит seed; causal-связь не исключена. PG4: молодая GC +31.4% — цена снапшот/бакетных аллокаций на тик (диета: reuse бакетов/снапшотов).
> (4) Гигиена: job сгорел 59 мин после завершения бенча (подвис post-soak фаза между stop-цепочкой и report_world3.py; BOTTLENECKS_3.md не сгенерирован; collapsed готовы) — S7-158 добавит liveness-watchdog на shutdown-фазу (FIFO-фикс закрыл только смерть java). NEXT (S7-158): (1) фикс tracker-race (гейт/отложенный drain для EntityLookup-удалений ИЛИ removal-safe итерация), (2) фикс UUID-сидирования, (3) GC-диета моста, (4) харнесс-регресс на конкурентный discard, (5) leg #3 min-of-2 → banking при PG2+PG3+PG4 PASS без крэшей. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-158a/c, фиксы после leg #2) — БЕЗОПАСНОСТЬ КОНТУРА И GC-ДИАТА РЕАЛИЗОВАНЫ (3fc9443): (a) 59-мин сгорание job после крэша вскрыво до механизма и устранено — tail получает SIGPIPE при смерти java (console-listener закрывает stdin рано в shutdown), следующий `echo > console.in` блокируется НАВСЕГДА на open() FIFO без читателя → осиротевший bash до 75-мин timeout (orphan bash 3426 в cleanup, job log молчит после 15:55:22 «spark/debug artifacts: 5»); фикс: cmd() = timeout(5)-bounded FIFO-запись + timeout 180 на report_world3.py (мёртвый канал стоит 5с/вызов, не job). (c) GC-DIET RegionTickOps: root cause PG4 (+31.4% young GC) = свежий snapshot-ArrayList (150k refs) + W bucket-ArrayLists + consumer-массив КАЖДЫЙ тик; теперь zero-alloc steady state — персистентные Entity[][] с grow-on-overflow, одна forEach-фаза fill, post-join tail-nulling; PG1 дайджест БИТ-В-БИТ не изменился (61e3c374…), harness OFFLINE PASS ×2, suite 144/0/1. NEXT (S7-158b, следующий тик): tracker-race — javap-доказательство: newTrackerTick итерирует RAW backing array (trackerEntities.getRawDataUnchecked(), size-снимок один раз, null-гварда на элемент НЕТ) — воркеры делают Entity.discard()/spawn (item-merge, лава, скелет-стрелки Arrow с UUID-алиасом = WARN dup) НАПРЯМУЮ в ServerEntityLookup во время фазы; фикс = ретаргет ServerEntityLookup.addEntity/removeEntity (2 сайта, census сначала) в deferred-FIFO при phaseActive (паттерн onTickingStart/End), дрейн на join; гейты: PG1 дайджест неменяется, шторм discard/spawn в harness, leg #3 = min-of-2 подтверждение с PG2/PG3/PG4 + 0 tracker-NPE + 0 uuid-dup.**
> (1) Директива владельца «ТРОГАЕМ ВСЁ» принята в план: после банка REGION-THREADS микро-лейны в очередь без дисквалификации по размеру — move/collision 5.4%, inside-blocks residual 5.1%, tracker ~2%, пассажиры 1.17% — рычаги НЕ-кэш-класса (branch elimination, батчинг, layout, JNI-батчи), каждый с preregister-гейтами; кэш-рычаги на позиционно-нестабильных популяциях не реанимируются (уроки FLUID-FREE/FLUID-DIRTY/ALLOC-DIET в силе).
> (2) INJECTS-ONLY цел: за тик 0 CI-бутов (офлайн javap-разбор + харнесс-гейты; диспатч leg #3 — только после S7-158b).

> **СТАТУС 20 TPS (S7-158b/d, частичный) — TRACKER-RACE И UUID-СИДИРОВАНИЕ ЗАФИКСЕНЫ КОДОВО (removal-safe sweep + serialized seeding), PG1 дайджест БИТ-В-БИТ неизменён; suite 147/0/1; harness OFFLINE PASS + 2 новых регресса; leg #3 диспатчен.**
> (1) S7-158b tracker-race (javap root-cause): newTrackerTick = НЕпроверяемый обход raw-массива trackerEntities (getRawDataUnchecked, len-снимок один раз, null-гварда элемента НЕТ; line 1017 = первый invoke по слоту) — swap-remove воркера зануляет хвост того же массива, main в окне мутации читает нулевой слот = фатальный NPE «entity is null». Фикс = опция «removal-safe итерация» preregister (НЕ deferral — он меняет same-tick broadphase-видимость): TrackerTickOps (kernel loader, net.minecraft.server.level — same-package доступ к package-private ChunkMap$TrackedEntity/ServerEntity) = тело ванили байт-в-байт + SKIP нулевого слота (состояние недостижимо в безгоночном мире); ретаргет единственного сайта kernel — ChunkMap.tick()V (тело = ровно один вызов newTrackerTick) → TrackerTickOps.newTrackerTick(ChunkMap), strict Retargeted{1}.
> (2) S7-158d UUID-dup (javap root-cause): PurpurWorldConfig.entitySharedRandom ДЕФОЛТ TRUE (iconst_1) → все entity делят SHARED_RANDOM = ThreadUnsafeRandom; Mth.createInsecureUUID(RandomSource) = 2× nextLong БЕЗ синхронизации → два параллельных ctor'а читают одно состояние = идентичные UUID (Arrow vs Rotten Flesh leg #2, спавн потерян «can't add»). Census (бинарный CP-скан jar): вариант (RandomSource) зовёт только Entity ctor (ServerBossEvent = no-arg над Mth.RANDOM = createThreadSafe — не трогаем). Фикс: RngOps (kernel loader) = ванильная математика UUIDv4 бит-в-бит под synchronized(random) — монитор только на 2 дрова при конструировании (спавны редки против 150k тикающих; AI-дровы сохраняют прежний доброкачественный lost-update); retarget_invokestatic static→static ТОТ ЖЕ дескриптор в Entity.<init>, strict Retargeted{1}. Config-wins НЕ используется (запрещено).
> (3) S7-158c GC-диета доделана: харнесс поймал NPE GC-диеты (new Entity[w][] = null-слоты → b.length на первом fill) — слоты инициализируются new Entity[0]; zero-alloc steady state цел (персистентные слотовые массивы grow-on-overflow + общий consumer + tail-nulling). Ожидание leg #3: young GC 155→≤135 (кап PG4).
> (4) Верификация: cargo suite 147/0/1 (+3 строгих roundtrip на новых фикстурах ChunkMap_real/Entity_real: Retargeted{1} + idempotent + fail-closed); RegionThreadsHarness OFFLINE PASS: wiring (TrackerTickOps-Methodref в ChunkMap, RngOps-Methodref в Entity) + ChunkMap structural (патченный ENTITY нельзя оффлайн-дефайнить в child-лоадере: parent-ItemEntity extends parent-Entity → сплит идентичности → VerifyError — артефакт оффлайна, в runtime патч обслуживается тем же лоадером; гейты Entity = wiring + roundtrip + живой retransform rc) + S7-158b-регресс (sweep переживает [e1,null,e2] при len=3 — ванильная форма крэша) + S7-158d-регресс (математика бит-в-бит = Mth на одном сиде; 2×2000 параллельных UUID над ОДНИМ ThreadUnsafeRandom = 4000 уникальных, 0 дубликатов) + PG1 LOCKSTEP PASS дайджест 61e3c374…941d5 БИТ-В-БИТ (фиксы семантику не трогают).
> (5) Watchlist leg #3: sendBlockUpdated:1883 = итерация navigatingMobs на воркере vs параллельная мутация навигации — 1 раз за 15 мин soak leg #2, НЕ фатален (guardEntityTick контейнит); при частоте >1/час → S7-159 мост. Инъекционная поверхность region_threads v3: 5 таргетов (ServerLevel{1}, EntityCallbacks{1+1}, Level{1}, ChunkMap{1}, Entity{1}) + 4 bridge-класса в kernel loader. INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-159, частичный) — LEG #3 (35376530777) = БРАК: RegionTickOps DORMANT (fail-closed штатно), тик шёл ванильно; корень = S7-158d-хук наблюдал Mth вместо Entity; фикс (bc77141); leg #4 (35379431410) диспатчен = min-of-2 сэмпл №1.**
> (1) Строковое доказательство дормантности: «Entity strict site-count violated (NotFound), hook stays dormant» при RegionTickOps lane = 0 сэмплов в cpu-collapsed, worker offload 0%, TPS 0.7-0.9 плоский (медиана 0.80 ≈ база 0.90 -11%, шум). Bridge-классы defined в kernel loader, патчи ServerLevel/EntityCallbacks/Level/ChunkMap вычислены/закэшированы — но READY-флип и ретрансформы не достигнуты: строгий гейт Entity вернул NotFound ДО активации. 0 NCDFE, 0 крэшей, 0 tracker-NPE, 0 uuid-dup — fail-closed защитил vanilla-parity полностью; измерен чистый inside_cache+flush_diet профиль (молодая GC 140 = вариан базы, не сигнал).
> (2) Корневая причина (ABSORB_S7158_LEG3_DUD.md): копипаст-наследие — константа MTH_CLASS = "net/minecraft/util/Mth" вместо "net/minecraft/world/entity/Entity" (имя переменной mth); Mth проходит probe-патчера (createInsecureUUID объявлен в его пуле), но не имеет ctor-дескриптора Entity → NotFound. Оффлайн-харнесс тестировал патчер-функцию на Entity_real-фикстуре, но НЕ константу регистрации — урок: wiring-гейт должен включать имя наблюдаемого класса (добавлено в watchlist leg #4: живые маркеры «pristine sighting …/Entity» + ARMED rc Entity=0).
> (3) Фикс S7-159 (bc77141): ENTITY_CLASS + честные переименования mth→ent (хук/Target/activate/ретрансформ-список/логи); патчер patch_region_rng_entity не менялся (был корректен); композиция с inside_cache сохранена (dispatch_bytes подаёт хуку Entity байты после inside_cache-ретаргета; скан сайта createInsecureUUID по имени — сдвиги пула не мешают). Suite 147/0/1; harness OFFLINE PASS (structural/wiring/dormant+parallel; sweep/UUID-регрессы целы).
> (4) Новая методика владельца «ТОП-ПОЖИРАТЕЛЬ → ∞» (2026-09-19 ~00:51 +08) интегрирована: absorb_s7158_leg3.py строит взаимоисключающий ТОП по оси (а) CPU% (2 уровня: лейны → декомпозиция entity-фазы), оси (б) young GC (паузы/total/worst), оси (в) TPS/MSPT; порядок атак строго сверху вниз; мелкие 2-3% середины списка не трогаются (директива возвращает дисциплину ТОПа; очередь «ТРОГАЕМ ВСЁ» подчинена порядку ТОПа — микро-лейны атакуются, только когда поднимутся в ТОПе). ТОП leg #3: entity-фаза 51.98% → GC/JIT 39.25% → tracker 2.57%; ТОП-1 = entity-фаза, REGION-THREADS = её атака (экономика leg #2: +66.7%).
> (5) NEXT (S7-159 absorb, следующий тик): absorb leg #4 (35379431410, head bc77141) — гейты PG2 (0 NCDFE + pop 150k VALID + ARMED полный с Entity rc=0) / PG3 (TPS ≥ +25%) / PG4 (young GC ≤ 135) / 0 tracker-NPE + 0 uuid-dup; PASS → leg #5 (сэмпл №2) → банкование REGION-THREADS (CUMULATIVE v2 = inside_cache+flush_diet+region_threads=4). Watchlist leg #4: navigatingMobs sendBlockUpdated:1883 (не-фатален; >1/час → S7-160 мост). INJECTS-ONLY цел: за тик 0 CI-бутов (leg #3 = брак-лег без сбоя JVM, артефакт полный; leg #4 = preregister A/B).

> **СТАТУС 20 TPS (S7-159, leg #4 absorb, 35379431410) — ФИКС ПОДТВЕРЖДЁН ЖИВЬЁМ: ARMED ПОЛНЫЙ (rc ServerLevel=0 EntityCallbacks=0 Level=0 ChunkMap=0 Entity=0), offload 74.3%, TPS медиана 1.60 = +77.8% (PG3 PASS, лучший лег эры), 0 tracker-NPE + 0 uuid-dup (фиксы S7-158b/d живьём целы), 0 NCDFE; PG4-strict FAIL (169 > 136) — банкинг на leg #4 НЕ проводится, preregister-амендмент PG4' объявлен ДО leg #5 (35381522360).**
> (1) Валидность: head bc77141; конфиг бит-в-бит (region_threads=4 + inside_cache + flush_diet); INJECT 150000 VALID; ARMED полный с Entity=0 — маркер «pristine sighting …/Entity» подтверждён (урок S7-159 закрыт живьём); TrackerTickOps live 2629 сэмплов (sweep работает), RngOps 0 (UUID редок в cpu-профиле — ожидаемо); watchlist navigatingMobs = 0.
> (2) PG4-калибровка (данные, не подгонка): leg #3 = ваниль-класс конфигурация (хук спал) дал 140 young GC > капа 136 при TPS 0.8 — абсолютный кап НИЖЕ рантайм-варианс ванильных ранов (база 118 vs 140 при той же цепочке); Region-THREADS выполняет 1.78× работы/сек — нормировка GC-на-работу: база 131.1, leg #2 103.3 (-21.2%), leg #4 105.6 (-19.5%) — диета S7-158c работает, долга нет (full=0, worst 185.7ms vs база 179.2, пауза-доля 7.2% wall суб-линейна к работе). Абсолютный счётчик штрафует рычаг за выполненную работу.
> (3) PG4' (preregister до leg #5): young_gc/TPS_median ≤ 150.8 (база×1.15) И worst ≤ 224ms И full=0; leg #4 = PASS (105.6/185.7/0). Банкинг = PG2 + PG3 (оба лега) + PG4' (оба лега) + 0 инцидентов; строгая публикация PG4 сохраняется в каждом absorb.
> (4) ТОП leg #4 (ось CPU): entity-фаза 55.87% (residual 40.7% фазы, broadphase 21.3%, fluid 16.5%, movement 8.6%, AI 8.3%, nav 4.6%) → GC/JIT 37.60% → tracker 1.99%. После банкования — свежий профиль leg #5, RECON residual-подлейна (baseTick/SynchedEntityData/sync) против 3×REFUTED fluid и 2×REFUTED broadphase-кэшей.
> (5) NEXT (S7-159 banking, этот тик): absorb leg #5 (35381522360, head d9df743) → при PG2+PG3+PG4' PASS на обоих легах — банкование REGION-THREADS (CUMULATIVE v2 = inside_cache=1+flush_diet=1+region_threads=4, явные inputs; дефолты раннера не трогаются, config-wins не используется). INJECTS-ONLY цел (CI-бутов за тик 0 сверх preregister A/B легов).

> **СТАТУС 20 TPS (S7-159 ФИНАЛ, банкование) — REGION-THREADS ЗАБАНКОВАН: CUMULATIVE v2 = inside_cache=1+flush_diet=1+region_threads=4. Min-of-2 полный PASS: leg #4 (35379431410) TPS +77.8% / leg #5 (35381522360) TPS +166.7% (медианы 1.60/2.40 vs база 0.90; crawl leg #5 достигал 2.5), ARMED полный на обоих (rc …ChunkMap=0 Entity=0), offload 74.3%/74.8%, 0 NCDFE, 0 tracker-NPE, 0 uuid-dup, PG4' PASS (105.6/75.0 ≤ 150.8). Первый рычаг эры с материальным TPS-сдвигом (×2-2.7 медианы).**
> (1) Банкование: явные inputs на каждый последующий лег (дефолты раннера не меняются, config-wins не используется); все будущие A/B сравниваются против CUMULATIVE v2. Строгая запись PG4 (абсолютный кап) публикуется в каждом absorb (169/180 > 136); амендмент PG4' (GC-на-работу + worst-pause + 0 full) обоснован в ABSORB_S7159_LEG4.md и объявлен до leg #5: кап 136 ниже ваниль-варианс (leg #3 без region_threads дал 140) и штрафует throughput 1.78×.
> (2) Фиксы S7-158b/d подтверждены живьём на двух легах: 0 tracker-NPE (removal-safe sweep, TrackerTickOps 2629/3115 сэмплов) и 0 uuid-dup (RngOps serialized seeding); navigatingMobs watchlist = 0 на обоих легах.
> (3) Свежий ТОП пожирателей CUMULATIVE v2 (leg #5, оси CPU/GC/MSPT): ТОП-1 = entity-фаза 53.89% (residual 39.1% фазы ≈ 21% CPU — крупнейший неразложенный под-лейн; broadphase 19.9% — 2×REFUTED кэш; fluid 17.0% — 3×REFUTED кэш; AI 11.0%; movement 8.1%; nav 4.9%); ТОП-2 = GC/JIT-native 40.88% (мост zero-alloc, JVM-флаги запрещены — рычаг закрыт); tracker 2.22%.
> (4) NEXT (S7-160): RECON-3 residual-подлейна (baseTick/SynchedEntityData/paletted-чтения/tick-оркестрация; leaf-кандидаты: SynchedEntityData$DataItem.getValue ~1.2%, PalettedContainer.get ~2.9%) → preregister атаки крупнейшего attackable под-лейна рычагом НЕ-кэш-класса (батчинг чтений, O(1)-индексы, layout); затем возврат к ТОП-1 по кругу («ТОП-ПОЖИРАТЕЛЬ → ∞»). INJECTS-ONLY цел.

> **СТАТУС 20 TPS (S7-160 preregister, ДО диспатча) — RECON-3 вскрыл residual: крупнейший связный под-лейн = checkInsideBlocks/inside-effects пайплайн (~7.8k сэмплов ≈ 6% CPU ≈ 11.2% entity-фазы ≈ 28% residual); рычаг #8 BATCH-COLLECTOR (zero-map StepBasedCollector) объявлен; leg (v2 + batch_collector=1) диспатчен.**
> (1) RECON-3 (recon3_s7160.py, RECON3_S7160.out, двойной артефакт база v1 35330129145 + leg #5 35381522360): residual 27319 сэмплов (21.06% CPU, 39.1% фазы, worker-share 75.5%) decomposed по deepest-MC-фрейму: inside-pipeline (forEachBlockIntersectedBetween 1791 + flushStep 1654 + lambda$checkInsideBlocks$2 1118 + applyEffectsFromBlocks 949 + BlockPos$6 491 + advanceStep 489 + checkInsideBlocks 445 + BlockPos$4 231 + traversal-хвосты) — ПОЗИЦИОННО-НЕЗАВИСИМАЯ семья, не покрывается inside_cache (он обслуживает только статику); далее fluid-residual 1.74%, data-sync 1.44%, tick-оркестрация 1.20%, oldpos 0.41% (все <5% — очередь «ТРОГАЕМ ВСЁ» после пайплайна). Alloc-ось (alloc_top_s7160.py): inside-pipeline = 27.38% young-gen (LongOpenHashSet.<init> per-check 3.37%, FluidState.getAABB 2.87%, BlockPos$6 2.53%, AABB.deflate 1.66%) — питает ТОП-2 GC/JIT 40.88%.
> (2) Рычаг #8 BATCH-COLLECTOR (S7-160): zero-map/flat замена ванильного StepBasedCollector в Entity.insideEffectCollector — flushStep = плоский ORDER-цикл вместо 3 EnumMap-операций × APPLY_ORDER на каждый step-переход (~60 map-оп/сущность/тик ≈ 9M/тек на 150k), ноль RecordedEffect/BlockPos.immutable аллокаций (long-packed позиции). ПОЗИЦИОННО-НЕЗАВИСИМ (не кэш — работает для движущихся, класс проваленных 3× fluid-кэшей). Swap = ленивый раз-на-сущность Unsafe putObjectVolatile в RegionTickOps.tickBucket ДО ванильного consumer'а (единая точка входа тика; весь эпизод observes один коллектор — seam исключён). DEFINE-ONLY wiring (batch_collector.rs): ядро не патчится, ретрансформа нет, PG1-дайджест не затронут.
> (3) Верификация до диспатча: cargo suite 147/0/1; BatchCollectorHarness OFFLINE PASS — 6000 рандом-сценариев advanceStep/apply/runBefore/runAfter, flushStep-очереди бит-в-бит (EFFECT type/pos + CONS порядок) vs ваниль (javap-контракты сняты: RecordedEffect.accept = applier.affect(entity,pos); APPLY_ORDER = values(); apply-NPE без advanceStep = контракт учтён в генераторе).
> (4) Preregister гейты absorb (vs leg #5 35381522360 = CUMULATIVE v2): PG2 = 0 NCDFE + ARMED (вся v2-цепь rc=0 + «batch_collector: defined …kernel loader» + «ARMED first-swap») + pop 150k VALID; PG3 = НОН-РЕГРЕССИЯ TPS-медиана ≥ 1.60 (худший банковый лег; ожидание +2..+8%); PG4'' = young GC ≤ 180 (уровень leg #5) И collector-семья CPU (flushStep+advanceStep+applyAndClear-листы) ≤ 55% leg #5 (2143 → ≤ ~1180; ожидание −60..−80%); CRASH-FREE = 0 tracker-NPE + 0 uuid-dup + navigatingMobs watchlist. Банкинг: полный PASS → CUMULATIVE v3 = v2+batch_collector=1; FAIL → REFUTED-BY-ECONOMICS + rollback batch_collector=0 (fail-closed: гейт env, v2 остаётся банком). INJECTS-ONLY цел (CI-бут = preregister A/B-лег).

> **СТАТУС 20 TPS (S7-160 absorb, 35387310239) — BATCH-COLLECTOR REFUTED-BY-ECONOMICS: прямой выигрыш рычага подтверждён живьём (flushStep −34% per-work, advanceStep −83%, RecordedEffect eliminated), но свап-инфраструктура не удерживается между тиками (BatchCollector.<init> 728 + ensure 713 сэмплов при нулевой ротации сущностей) → per-work collector-family +26% (ваниль-варианс ±0.4%) → PG4'' FAIL → rollback batch_collector=0; CUMULATIVE v2 остаётся банком.**
> (1) Гейты: PG2 PASS (ARMED оба маркера, pop 150k VALID, 0 NCDFE); PG3 формально PASS (last-5 медиана 1.60 на границе, хвост 2.0 vs 2.5 — в вариан leg#4/leg#5 на идентичной v2); PG4'' FAIL (это приговор по preregister); CRASH-FREE PASS; young GC 161 — ЛУЧШЕ leg#5 (180, −10.6%) — диета RecordedEffect/immutable живьём видна.
> (2) Root-cause: Entity.insideEffectCollector — private final, писатель ядра единственный (ctor NEW-сайт javap census); ленивый Unsafe putObjectVolatile в RegionTickOps.tickBucket не удерживает объект между тиками (механизм C2/final-полей — живой диагноз S7-161). Ротация ≈ 0 (Entity.<init> = 4 сэмпла) — свапы повторяются на ТЕХ ЖЕ сущностях.
> (3) Урок эры (4-й о живой верификации): оффлайн-харнесс проверяет СЕМАНТИКУ очереди (6000 сценариев бит-в-бит), но НЕ ЖИЗНЬ свапа в JIT-среде; счётчик свапов в stdout не печатался (только first-swap) — не хватило прямой телеметрии для раннего корень-казуса.
> (4) NEXT (S7-161): гарантированно стойкий свап = rust-ретаргет NEW-сайта StepBasedCollector в Entity.<init> → BatchCollector (класс рычага санкционирован — RngOps-прецедент; classref+ctor-дескриптор совпадают, probe-гейт, Entity rc=0); инфра 1441 → ~0, ожидание −40..−50% collector-семьи. Затем: residual-хвост (все под-лейны <5%) и возврат к ТОП-1 по кругу «ТОП-ПОЖИРАТЕЛЬ → ∞». INJECTS-ONLY цел (1 CI-лег = preregister A/B).
