# RESEARCH-459-P42 — Goal canUse-пре-гейт: sense-memo (TASK-459-67, WILD, закон 11)

Идея **ID-P42** (карточка `RESEARCH-458-P.md`, ветка `round-458p-ideas`): memo-хэш требуемых
сенсов на goal; `canUse` пропускается, если сенсы не менялись (superset-инвалидация по
sense-событиям). Лейн: goalops 1-2пп × 30-40% → **+0.4-0.8пп**. Статус сабагента: SCAFFOLD
(вайринг-заготовка + research; STRICT-off до оракула).

## 1. Механика (ground truth)

### 1.1 Ванильный сайт (javap-канон, purpur-1.21.10; GoalOps.java javap-транскрипция)

- `Mob.serverAiStep()V` содержит РОВНО 2 сайта `invokevirtual GoalSelector.tick()V`
  (@161 targetSelector, @183 goalSelector — чётные тики) и РОВНО 2 сайта
  `invokevirtual GoalSelector.tickRunningGoals(Z)V` (@113/@136 — нечётные тики, оба false).
- Тело `GoalSelector.tick()V` (фазы): `goalCleanup` (stop по disabled-флагам /
  `!canContinueToUse()`; purge lockedFlags) → `goalUpdate` → `goalTick`
  (`tickRunningGoals(true)`).
- **Целевой сайт P42 — фаза `goalUpdate`**: последний предикат перед `start`:
  `if (!g.isRunning() && !goalContainsAnyFlags(g, disabled) && goalCanBeReplacedForAllFlags(g, lockedFlags) && g.canUse())`.
  `Goal.canUse()` пере-спрашивается **каждый чётный тик** для каждой не-running цели —
  дорого для targeting-целей (NearestAttackableTargetGoal.canUse → findTarget →
  getNearestEntity = sense-чтение полного скана кандидатов).
- Paper-сплит: на нечётных тиках `canUse` не зовётся вообще (только
  `tickRunningGoals(false)` для `requiresUpdateEveryTick`).

### 1.2 Идея memo

Per-goal слот: `(senseSigHash: long, memoUse: boolean, armedTick: long)`, где
`senseSigHash` = FNV-1a над (sense-эпоха моба, идентификаторы требуемых сенсов цели).
`goalUpdate`-пре-гейт: если `senseSigHash` не изменился с последней оценки →
`canUse` НЕ вызывается, переиспользуется memoized boolean (сенсы не менялись ⇒
canUse — чистая функция от (внутр. состояние goal, sensed-мир) ⇒ вернула бы то же
значение = **superset-инвалидация**: любые записи сенсов перехвачены sense-плоскостью
(SenseOps nearestEntityGate — единый chokepoint, RESEARCH-A-439-SENSE) и поднимают
эпоху/version; «эпоха не двигалась» ⊇ «canUse-входы не менялись»).

Паритетный якорь ванили: `Sensing` мемоизирует LOS «ровно на один тик» и чистится в
верху `serverAiStep` ([источник 2]) — sense-memo поверх goal-селектора продолжает тот
же приём, но с явной инвалидацией по событиям вместо безусловной очистки.

### 1.3 Внешние опоры (web, ≥2 источника)

| # | URL | Что даёт |
|---|-----|----------|
| 1 | https://docs.pufferfish.host/setup/pufferfish-fork-configuration/ | **DAB** («DAB is an optimization that reduces the frequency of brain ticks»): `freq = distanceToPlayer^2 / 2^activation-dist-mod`, кап `max-tick-freq`, `blacklisted-entities`; прямо фиксирует, что DAB «impacts the AI goal selector behavior of all entities». Прецедент индустрии: частота AI-оценок гейтится свежестью/близостью входов, а не фиксированным деадбандом — наш memo-gate делает то же по sense-событиям, сохраняя порядок canUse. |
| 2 | https://minecraftdocs.dev/systems/entities/ai-goals-and-brains | Канон GoalSelector (verified vs 26.2): `availableGoals` — insertion-ordered set `WrappedGoal` + lock-table + disabled-flags; «A goal is asked `Goal.canUse` **every other tick**, staggered across mobs by `tickCount + id`» (исключение — первые два тика моба); фазы goalCleanup/goalUpdate/goalTick; `Mob.updateControlFlags` каждые 5 тиков пушит флаги только в один селектор; `Sensing` — «memoised for exactly one tick … cleared at the top of `Mob.serverAiStep`». Для нас: (а) канал изменений флагов/leash вне sense-плоскости — у P42 его нет, флаги читаются живо как ваниль; (б) stagger `tickCount+id` не ломается — memo сидит ТОЧНО на вызове canUse внутри goalUpdate, порядок и частота вопросов не меняются. |

## 2. Сайты записи сенсов (ложный-отрицательный ЗАПРЕЩЁН)

Все пишущие сайты сенсов перехвачены sense-плоскостью (существующий мост):
- `ServerEntityGetter.getNearestEntity(...)` — body-swap на `SenseOps.nearestEntityGate`
  (14-байт straight line, `patch_sense_nearest_entity`): 4 getNearestPlayer-варианта +
  getNearestEntity(Class/TagKey) депегаются сюда; NearestAttackableTargetGoal.findTarget
  зовёт напрямую. Единственный источник новых «целей» для canUse-целей.
- `senseEpoch(tick, idTop, players, nearest)` — bulk-JNI пишет колонку nearest[] и
  поднимает глобальный VERSION (seqlock, sscan_snapshot reader-контракт) — вот он,
  ready-made источник sense-СОБЫТИЙ для инвалидации.
- Скрытые мутации сенсов ВНЕ плоскости (мозги-v2, сторонние плагины через NMS) —
  единственный риск ложного-отрицательного; контрмера: **selfTest-инвариант** —
  1/N-тик форс-реоценка canUse поверх memo (расхождение → disarm навсегда + лог),
  STRICT-off до оракула.

## 3. Дизайн scaffold (этот коммит)

- `src/goal_sense_memo.rs` — rust-модуль вайринга (подключён в lib.rs, dormant):
  STRICT-гейт `cmp459_p42` (изолированный флаг — в семейства-носители НЕ добавлен,
  пустой/чужой флаг = ваниль бит-в-байт), план ретаргета сайта goalUpdate
  (`invokevirtual Goal.canUse()Z` внутри `GoalSelector.tick()V` → static
  `GoalOps.canUseMemoGate(Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)Z`,
  receiver-prepended stack-identical — по канону goal_selector.rs), pure-функции
  сигнатуры/инвалидации + юнит-тест (FNV-1a mixing, superset-инвалидация, порядок).
- `goalops/net/minecraft/world/entity/ai/goal/GoalOps.java` — АДДИТИВНЫЙ stub
  (существующие tickGate/tickRunningGate не тронуты): memo-слот-структуры,
  `canUseMemoGate(WrappedGoal)` — пока fail-closedPassthrough = `g.canUse()`
  (семантика ванили даже при случайном ретаргете), `noteSenseEvent(long version)` —
  hook под sense-плоскость, `fnv1a`-хелпер, selfTest-счётчик. Блоб пересобран
  `scripts/build_goalops_ops.sh` (major 65, flat==nested — ×93-дисциплина).

### NCDFE-канон (уроки ×448 / bridge_prelist)
1. javap ground truth ДО ретаргета: сайт считается найденным только javap-снапшотом
   (счётчики (2,2) для tick/tickRunning в serverAiStep — зафиксированы goal_selector.rs).
2. LOADABILITY: блоб --release 21 (major 65) = JVM-мажор ядра; define в kernel-loader
   (тот же пакет, что GoalSelector); расхождение мажоров = dormant, не падение.
3. define_class с null/boot loader до загрузки kernel-классов = NCDFE-ловушка —
   активация только после `find_class(GoalSelector)` + boot-marker (канон activate()).
4. Хук ничего не предполагает о порядке цепочки: compose на ПРИНЯТЫХ байтах,
   fail-closed pass-through на любой Err.

## 4. Δ и гейты

- Лейн: goalops-хвост 1-2пп (профили тика-458: goalUpdate/canUse-доля при 150k-популяции);
  захват 30-40% (memo бьёт только не-running цели со стабильными сенсами —
  пассивы/idle-монстры) → **прогноз Δ +0.4-0.8пп** к ноге.
- Пре-registered гейты (закон 13/16): ARM-пруф (маркер `cmp459_p42` в server-stdout)
  обязателен; min-of-2/3 ноги против round-459-anchor-* (pair Δ≤50k, band 6.0-9.5M);
  selfTest-mismatch = нога невалидна; STRICT-off до оракула-харнеса GoalOps.

## 5. Риски

| Риск | Митигация |
|------|-----------|
| Скрытые мутации сенсов вне sense-плоскости → ложный-отрицательный (цель не стартует когда должна) | selfTest-реоценка 1/N тиков + disarm при расхождении; STRICT-off до оракула |
| canUse с side-effect'ами (stateful-цели) | v1 whitelist: только targeting-семейство (canUse = чистый sense-чтение); остальные цели идут ванилью |
| memo на running-целях не нужен | gate только в goalUpdate (не-running ветка) — как ваниль |
| Полурасставленный мост (TASK-402-F) | изолированный флаг cmp459_p42, dormant = байт-в-байт ваниль; ARM-маркер grep до вердикта |
| Stale-блоб (×93) | flat==nested в чеке; пересборка build_goalops_ops.sh в этом коммите |

## 6. Источники

1. Pufferfish Server Fork Configuration — DAB: https://docs.pufferfish.host/setup/pufferfish-fork-configuration/
2. How Java Minecraft Works — AI: goals and brains: https://minecraftdocs.dev/systems/entities/ai-goals-and-brains
3. Локальный javap-канон: goalops/.../GoalOps.java (iter-2 транскрипция), src/mobs_sense.rs
   (sense-плоскость + senseEpoch), RESEARCH-A-439-SENSE.md (chokepoint getNearestEntity).
