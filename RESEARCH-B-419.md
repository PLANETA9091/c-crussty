# RESEARCH-B-419 — TASK-419-B: sensing+brain ПОДСИСТЕМА (закон 6 v17, lever cmp419_sense)

База: origin/master fbb06e3 (первый мерж эры: cvs⊕queryplane, +36.2% pair).
Дата: 2026-09-22 ~19:20 UTC (03:20 +08). Ночь: пул ~10пп, вердикты pair-by-runner vs fresh якоря-419.

## 0. СВЕЖИЙ ПРОФИЛЬ ЯКОРЕЙ-419 (главная поправка чисел BOTTLENECK-419)

Якоря-419 (vanilla, lever_flag="", fbb06e3, X150K pop 105k items/30k hostiles/15k passives, fp=4, band in):
- 35770236750 anchorb: TPS-окно [1.2,2.1,2.4,2.8,2.9] med 2.4, cpu_index 8 541 939, MSPT 366.65
- 35770245316 anchorc: TPS-окно [1.5,1.9,2.1,2.6,2.6] med 2.1, cpu_index 6 971 994, MSPT 409.05

Декомпозиция cpu-collapsed anchorb (112 737 сэмплов):

| лейн | % wall | разбор |
|---|---|---|
| Brain.tick | **1.04%** | startEachNonRunning 0.51 (F2-линза уже в базе!), tickEachRunning 0.17, tickSensors 0.09, forgetOutdated 0.01 |
| GoalSelector.tick | **11.81%** | WrappedGoal.canUse 5.25 (NATG 1.83 / RemoveBlock 0.85 / Zombie$1 0.58 / Avoid 0.48 / Tempt 0.28 / stroll 0.22), WrappedGoal.start 2.57 (RandomStroll→moveTo 2.01 = NAV-лейн), RunningGoals 1.34, итератор fastutil 1.1, goalContainsAnyFlags 0.53 (getFlags 0.40) |
| getEntitiesOfClass | **3.54%** | EntityLookup.getEntities 3.52 = section-walk → СНЯТСЯ queryplane на моём носителе (cmp417_bq в STRICT-OR) |
| TargetingConditions+getNearestPlayer+Sensing+clip | ~0.8% | hasLineOfSight 0.16, clip 0.10, BlockCollisions 0.26, conditions 0.29, nearestPlayer 0.21 |
| serverAiStep subtree | 13.41% | весь AI-кластер (aibatch 1/4 уже в базе под композит-флагом) |

**ПОПРАВКА**: строка BOTTLENECK-419 «Brain.tick ~6.9% + сенсорные сканы ~5-7%» = устаревшая
GOAL-строка ×112 эры S7-98 (ДО aibatch/stagger). На свежих якорях master'аBrain.tick = 1.0%,
сенсорные сканы = 1.5-3.5% (из них 3.52% section-walk уже снят queryplane). НО т.к. Brain.tick
вызывается ТОЛЬКО из customServerAiStep брейн-мобов (javap: Mob.serverAiStep НЕ зовёт Brain.tick;
единственный вызов = Axolotl/etc.), а поголовье X150K (zombie/skeleton/spider/creeper/husk/drowned/
cow/pig/sheep/chicken) — goal-мобы с ПУСТЫМИ мозгами,Brain.tick на сцене = натуральные villager'ы мира.

## 1. Что УЖЕ в master (НЕ дублировать)
- aibatch (cmp406_aibatch): golden-window N=4, 3/4 мобов skip serverAiStep — ГОЛ-селектор/brain/сканы ×0.25 на носителе
- sscan (cmp406_sscan): despawn-скан bulk-колонкой
- queryplane (cmp410_eindexq…cmp417_bq): getEntitiesOfClass в NearestAttackableTargetGoal.findTarget + AvoidEntityGoal.canUse
  → eq_epoch bulk (1 JNI/тик: soa[] frozen-колонки + head[]/next[] цепи) → per-query java walk без JNI
- F2-линза (brainhook, S7-114): startEachNonRunningBehavior → BrainOps flat-snapshot (в master ВЕЗДЕ, без флага — база якорей её несёт)
- stagger/meganav/navpool/eqsnap/soa/tickplane/items/collide-batch — остальные плоскости композита

## 2. Экосистемный ресёрч (batched AI sensing / SoA brain / memory arena)
- **Lithium `goalSelector`-оптимизации**: дедуп флаг-проверок (OptimizedSmallEnumSet — уже в moonrise),
  замена per-goal O(flags) переборов; `avoid redundant target checks`.
- **Pufferfish DAB / DEAR**: rate-batching AI-планов (у нас уже aibatch N=4 — дозу НЕ трогаю без владельца).
- **Lithium `targets`/`raid`-планы**: кэш кандидатов таргетинга с инвализацией по позиции — у нас
  урок FLUID-DIRTY: X150K позиционно-нестабильна → кросс-тиковый кэш результатов = 0 хитов. PARK.
- **Memory-arena для brain**: ExpirableValue.hasExpired — чистая таймстамп-математика; поголовье сцены
  goal-мобы (пустые Brain) →/memory-лейн на сцене ≈0.01% (forgetOutdated). PARK (не окупается).
- Вывод: незакрытая остаточная поверхность sensing+brain на носителе = **per-query walk
  queryplane-снапшота в java** (цепи head/next + frozen-колонки) — перевожу РОВНО её в rust-дата-плейн.

## 3. ДИЗАЙН: SENSE-PLANE (rust-крейт + ОДИН bulk-JNI/тик, вся логика+данные в rust)

### 3.1 TARGET-SCAN ARENA (sensing getEntities-of-class через снапшот — глубокий слой)
Ныне: eq_epoch (rust, 1 JNI/тик) пишет java-массивы soa[]/head[]/next[]; per-query java идёт цепями
(head→next, рандомный доступ по id на КАЖДОГО кандидата).
Сенс-плейн: В ТОЙ ЖЕ эпо́хе (отдельный bulk-нататив `senseArenaEpoch` сразу после eqEpoch под java
EPOCH_LOCK — по-прежнему ОДИН bulk-переход плоскости на тик) rust строит:
- `arena[]` — плотная сцена id-кандидатов, per-bucket CONTIGUOUS (порядок = порядок цепи: id-descending
  внутри бакета — 2-проходный counting-sort по Тому же java-parity хэшу floor(x/16)/floor(z/16));
- `arenaOff[]` int[CELLS+1] — префикс-границы бакетов.
Per-query java (EntityGoalQueryOps.eqQuery под STRICT-OR cmp419_sense): rect-проход читает
`arenaOff[h]..arenaOff[h+1]` — ПОСЛЕДОВАТЕЛЬНЫЙ int[]-проход вместо next[]-цепочки; кандидаты, порядок
и frozen-колонки ИДЕНТИЧНЫ (оракул-тест в rust: arena-слайс == chain-walk поэлементно).
Эффект: −1 рандомный deref на кандидата + линейная локальность на walk-этапе (самая горячая часть
остаточного sensing-лейна на носителе).

### 3.2 STRICT-OR-композит (мандат)
cmp419_sense добавлен в ВСЕ гейты мостов (rust: stagger/mobs_grid/mobs_sscan/tickplane/nav_plane/
items_index/items_manager/queryplane/entity_query/collide_batch/mobs_soa/mobs_manager/mobs_ai;
java: EntityGoalQueryOps/QueryPlaneOps/MobPushOps/ItemEntityManager/MobAiOps/MobScanOps) →
моя нога армит ПОЛНЫЙ композит master (+36.2% pair база) ⊕ sense-plane. Пустой/чужой флаг = ваниль
бит-в-байт (закон 4); маркеры ARM/EFFECT каждой плоскости + мой sense-маркер.

### 3.3 Parity-контракт
- arena == chain-walk:Same candidate set, same order (id-desc per bucket), same frozen columns,
  same rect-обход → список getEntitiesOfClass бит-в-бит (оракул в cargo test + selfTest-маркер).
- Порядок кандидатов = документированный delta-класс queryplane (getNearestEntity nearest-pick,
  тай-брейк по порядку) — не меняю, только сохраняю.
- FAIL-CLOSED: senseArenaEpoch ERR_STRUCT → java навсегда vanilla-цепи; ERR_RANGE/cold → vanilla
  этот тик; probe-мисматч → disarm; sites-гейт как у queryplane.

## 4. Что осознанно НЕ делаю (и почему)
- Brain.tick→rust полностью: бихевиоры = java-объекты (tryStart/tickOrStop/RNG),bit-for-bit перенос
  невозможен; остаточная цена на сцене 0.78-1.04%, из них 0.51% уже F2-линза. Пустые мозги goal-мобов
  НЕ тикают вовсе (javap: только customServerAiStep брейн-мобов).
- LOS/raycast-плейн: нужна block-дата (java); кросс-тиковый кэш = FLUID-DIRTY-урок (0 хитов на X150K).
- GoalSelector-сконфолдинг: RNG side-effects (canUse nextInt) — RECON-8 PARK подтверждён.
- RandomStroll.start 2.01% → PathNavigation.moveTo = NAV-лейн (meganav, не мой вектор).
- Доза aibatch N: 4→8 = тюнинг отклонения-дозы без санкции — не трогаю.

## 5. Метрики успеха (чек-лист вердикта)
ARM (cmp419_sense маркеры всех плоскостей + senseArena EFFECT) + grep AIOOBE=0 + band 6.0-9.5M +
pair-by-runner vs якоря-419 (35770227563/35770236750/35770245316) + min-of-3 + лейн-проверка
(елейн getEntitiesOfClass-walk/NavSelector вниз, никакой лейн вверх) + selfTest + javap flat==nested +
polls-стабильность + parity STRICT-OR + RAM/GC без регресса. БАР ≥+20% pair-stable (мандат).
