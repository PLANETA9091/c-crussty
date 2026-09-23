# EPOCH-2 — СВОДНАЯ КАРТА РЕШЕНИЙ A/B/C (финализация, полные пины)

Дата: 2026-09-20 (+08) · TASK-359 · Составил: RECON-35 финализирующий тик
Статус: диспатчи точечных рычагов ОСТАНОВЛЕНЫ; решение владельца A/B/C не поступило
Истина выше этого дока: docs/GOAL_20TPS_MINESSHIELD3.md (GOAL) + CLAIMS.md

## 0. Рамка

- Сцена X150K: 9216 форс-чанков, 150k сущностей (мобы как с игроками),
  fp4/300s/150k/seed42/r640/xmx10G. Цель миссии: 20 TPS при строгой ванильной
  парити (median-exact).
- ПРОТОКОЛ v8-REGRESSION (санкция владельца, GOAL ×31): ДВОЙНОЙ БАР — обе оси
  ≥+10% vs ANCHOR-SLOW (s7184: median5 1.60 @ 6680195) + min-of-2; широкий
  банд раннеров 6.0M..9.5M; диспатчер лейна = ТОЛЬКО dispatch_s7189.py.
- Полное картирование сцены (RECON-25..29 + дриллы 30..35) доказало: точечный
  архитектурный рычаг с ДВОЙНЫМ БАРОМ на текущей архитектуре НЕ СУЩЕСТВУЕТ.

## 1. ВАРИАНТ A — КУМУЛЯТИВНАЯ ЭПОХА-2 (системный батч в одном CI-леге)

Статус: ИСЧЕРПАН (9 док-закрытий). НЕ РЕКОМЕНДОВАН.

| компонент | клейм | факт (кросс s7194/s7189) | вердикт | пин |
|---|---|---|---|---|
| GC/jvm (G1+barriers) | — | 33.4/32.5% сцены | ЗАКРЫТ: потолок TPS-конверсии <10% (паузы 5.9-6.5% окна; zeroin-fluid −5.8% CPU → TPS −12.5% наоборот) | RECON26_GC_LANE_VERDICT.md, RECON-24 alloc-атрибуция |
| jdk-collections/serde | 13.1/13.7% CPU | единственный домен ≥5% (visit-set 7.2/8.3%) = внутренность REFUTED #9 | ЗАКРЫТ | RECON25_JDK_COLLECTIONS.md |
| entities/mobs-tick | 13.9/13.7% | fractal, макс под-лейн 3.4-3.7% | ЗАКРЫТ | RECON27_ENTITIES_DRILL.md |
| tickBucket-оркестрация | — | чистая оркестрация 1.1% | <5% микро-зона | RECON-28 (recon28_tickbucket_decomp.py) |
| broadphase getEntities | ~5% | 4.90/5.14% на пороге; потолок буста ~5% | <10% бара; мемоизация законна, но не самостоятельный рычаг | RECON29 + RECON30_BROADPHASE_MEMO_FEASIBILITY.md |
| volatile/VarHandle-диета | ~3.4% | sync-семья уже plain; chunk-семья конкурентна по необходимости; честный потолок 0-1% | ЗАКРЫТ (6-е док-закрытие) | RECON31_VOLATILE_DIET_VERDICT.md |
| inside-gate диета | ~3.1% | семья 11.28/11.65% (discovery 9.08/9.36 + effects 2.20/2.29); легальный рычаг = solid-bitmask, TPS-конверсия ~0 | ЗАКРЫТ как option-A-компонент → переехал в option B как флагман | RECON32 + RECON33 |
| chunk-read диета | ~3.6% | **7.36/7.54% (клейм занижен ~2×)**, семья = инфраструктура: fluid ~50% (REFUTED-зона #10), movement ~22% (лейн #14 REFUTED), inside ~10% (dormant #15), pathfinding ~1.4% | ЗАКРЫТ (9-е док-закрытие, RECON-35) | RECON35_BLOCKREAD_CONSUMER_VERDICT.md |

REFUTED-эмпирика (6 рычагов, структурный эффект подтверждён — TPS НЕ растёт):
#9 flat_traversal, #10 zeroin-fluid, #11, #12 alloc-diet (TPS −17..−6%),
#13-park, #14 travel (v2a: −21.6%/−18.8%; v2b: −1.5%/+0.0% —
LANE14_VERDICT.md, ABSORB_S7189.md).

Вывод по A: кумулятивный потенциал системной группы ~9-12% сцены при
TPS-конверсии ~0 (эмпирика 6 REFUTED-ног) → ДВОЙНОЙ БАР недостижим даже
батчем. Реализация оплачена вердикт-доками, патчи не имеют смысла.

## 2. ВАРИАНТ B — КОРРЕКЦИЯ ПРОТОКОЛА (цель min-MSPT) — ФЛАГМАН ГОТОВ

Статус: РЕАЛИЗАЦИЯ + ОРАКУЛ ЗАВЕРШЕНЫ, lever dormant; активация = env-флаг.

Флагман: **#15 INSIDE-BITMASK** (крупнейший легальный остаток сцены,
сырой потолок ~9.1% CPU — воздушная доля inside-discovery движущихся
сущностей; effects-apply 2.20/2.29% не трогается = парити).

Пины:
- Контракт: RECON33_INSIDE_BITMASK_CONTRACT.md +
  scripts/bench4_recon/recon33_inside_bitmask_contract.py +
  contract-inside-bitmask-s7194/ (8 javap-дампов). Точка вставки:
  Entity.checkInsideBlocks после makeBoundingBox(to).deflate(1e-5);
  pre-gate = hasOnlyAir() секций deflated-AABB (прецедент ванили:
  LevelChunk.getFluidState fast-path); fluid-секции НЕ скипаются;
  debug-поток гейтится; visitedBlocks/бюджет на air-пути не наблюдаются
  (median-exact).
- Реализация: коммиты 6eb3274 + 1c703f7 (InsideBitmaskOps.java: swept-hull
  BB(from)∪BB(to)+1b, package-private sweptHullInto + ThreadLocal scratch
  — hot path zero-alloc; classfile.rs patch_inside_bitmask: единственный
  сайт applyEffectsFromBlocks bc 58..64, strict sites==1, 3B→3B
  receiver-first; inside_bitmask.rs probe-then-patch armState()==ARMED;
  entity_compose stage 1b; фоллбэк = MethodHandle.invokeExact исходного
  private CIB(List), static-init fail-closed). Dormant: env
  CRUSSTY_INSIDE_BITMASK=0 по умолчанию.
- Оракул: dea9de8 → ORACLE_INSIDE_BITMASK_PASS.md +
  scripts/run_inside_bitmask_lockstep.sh (одна команда) — [1] structural+ARM
  на реальном jar s7194; [2] hull-superset: 1,000,000 адапверсариальных
  кейсов = 210,513,534 позиций РЕАЛЬНОГО forEachBlockIntersectedBetween ВСЕ
  внутри домена РЕАЛЬНОГО sweptHullInto; [3] section truth: 20,000
  последовательностей / 249,217 setBlockState — hasOnlyAir/nonEmptyBlockCount
  бит-согласованы. Честный резидуал: live chunk-map wiring/debugSubscribers/
  живой effects-поток = гейты CI-лега.
- CI: green на dea9de8 + 5fd1f64 (run 35481721020 / 35481704496).

План при санкции B: (1) протокол банкинга меняется на скорость-ось (min-MSPT,
нормализованная дельта ≥+10% при TPS не-регрессе ≥ якоря-0); (2) CI-лег
с CRUSSTY_INSIDE_BITMASK=1 (диспатчер dispatch_s7189.py, широкий банд
6.0..9.5M); (3) absorb одной командой, вердикт по гейтам PG-T1..T5; (4) при
GREEN — min-of-2 подтверждение → banking. Опционально комбинировать с
уже реализованным dormant-кодом travel_diet v1/v2a/v2b (arm через yml) как
второй рукой под min-MSPT — по отдельности REFUTED под двойным баром, под
min-MSPT суммируются с #15.

Риск B: TPS-конверсия CPU-экономии ~0 по эмпирике диет (6 REFUTED) — но B
по определению не требует TPS-роста, только min-MSPT. CPU-эффект ~9.1%
потолка подтверждён профилем (discovery 9.08/9.36%).

## 3. ВАРИАНТ C — СМЕНА УРОВНЯ (Rust/JNI-планировщик / off-thread world-tick)

Статус: вне текущего цикла тиков по объёму; предпосылки частично готовы.

Что уже есть (инфраструктура ядра):
- Rust-ядро с patcher-стеком classfile.rs (прецеденты: travel_diet.rs,
  inside_bitmask.rs — probe-then-patch, arm-стейты, cargo-оракулы 201/201);
- jvmti_bindings (JNI-мост), entity_compose стадии, dispatch/absorb-тулинг
  (dispatch_s7189.py / absorb_s7189.py), локстеп-оракулы (v2a ×29, #15
  1M-кейсов);
- банк v3: inside_cache + flush_diet + region_threads + batch_collector.

Что требует C (объём):
- планировщик сцены на Rust-стороне (entity-tick scheduler, off-thread
  world-tick) — несравнимо больше одного тика: спека (S7-номер), JNI-граница
  владения сущностями, parity-оракул локстепа для ПОРЯДКА тиков, гейты;
- ПРЕДПОСЫЛКА: RECON22_BANK_V3_RACE.md — race navigatingMobs ЛАТЕНТЕН
  в банке v3 (фундаментально, стохастический s7180-class throw); S7-170
  кандидат задокументирован — threading-экспансия без его закрытия умножает
  окно гонки. Парити-риск прегистерить.

Оценка: C = отдельная эра (множество тиков: спека → JNI-контракт → оракул
порядка → CI-леги). Единственный путь к качественному (не процентному)
росту TPS на X150K, т.к. CPU-оси Java-уровня исчерпаны (раздел 1).

## 4. МАТРИЦА РЕШЕНИЯ (владельцу)

| | A (системный батч) | B (min-MSPT, флагман #15) | C (Rust/JNI уровень) |
|---|---|---|---|
| состояние | исчерпан доками | код+оракул готовы, dormant | спека не начата |
| ожидаемый эффект | ~0 TPS (эмпирика) | до ~9.1% CPU ↓ MSPT | качественный скачок TPS |
| объём до вердикта | 0 (закрыт) | 1 CI-лег + min-of-2 | эра из многих тиков |
| парити-риск | — | низкий (median-exact пины + 1M-оракул) | высокий (порядок тиков) |
| зависимость | — | только санкция | S7-170 (race) приоритет |

Рекомендация (RECON-29×32×35): санкционировать B (одна команда активации)
и параллельно открыть спеку C с S7-170 как первым тиком.
