# RESEARCH-459-P47 — GoalSelector transition-diff batch (iter-3)

TASK-459-72, WILD-агент закона 11, тик-459. Карточка: ID-P47 из RESEARCH-458-P.md
(origin/round-458p-ideas @ fetch 2026-09-26). Веткa: round-459-p47 (worktree
/tmp/wt459-p47, sparse: src/cplug-abi/cplug-sdk/goalops/.github/scripts).

## Идея (карточка, 1:1)

iter-2 flat-проходы tick/tickRunningGate ВЗЯТО как носитель. ДОБАВЛЯЕТСЯ:
stop/start переходы батчами с отложенным пересчётом флагов управления
(1 пересчёт/тик вместо per-goal), insertion-order стоп/старт сохранён.
Прогноз: +0.3-0.6пп goalops-хвост. Оракул: GoalOps-харнесс, чёт/нечёт
Paper-сплит тестом; регрессия-гейт — parity tickRunningGoals(Z).

## Javap ground truth (purpur-1.21.10, канон goal_selector.rs iter-2)

- `Mob.serverAiStep()V` — РОВНО 4 сайта: `invokevirtual GoalSelector.tick()V`
  x2 (@161 targetSelector, @183 goalSelector; чётные тики) и
  `invokevirtual GoalSelector.tickRunningGoals:(Z)V` x2 (@113/@136; нечётные,
  оба аргумента false) — Paper-сплит.
- Ваниль `GoalSelector.tick()`: фазы goalCleanup → lockedFlags purge →
  goalUpdate → goalTick; stop/start/canUse/canContinueToUse/tick — в ТОМ ЖЕ
  insertion-order (ObjectLinkedOpenHashSet). Фазы уже флэтнуты iter-2;
  iter-3 меняет ТОЛЬКО учёт переходов: сбор stop/start в дифф-батч +
  отложенный пересчёт флагов управления (MOVE/JUMP/LOOK/TARGET) 1/тик.
- Контракт порядка: ваниль применяет каждый stop/start НЕМЕДЛЕННО в порядке
  обхода; батч с отложенным пересчётом флагов сохраняет наблюдаемую
  семантику, т.к. флаги управления пересчитываются в ЕДИНСТВЕННОЙ точке тика
  (та же точка, что ванильный updateControlFlags после goalSelector.tick в
  serverAiStep), а промежуточные состояния флагов между переходами одного
  тика не читаются никем вне тика (однопоточный server tick).

## Δ-математика (capture-math)

- Хвост goalops по карточке-носителю: goalops-хвост прогноз +0.3-0.6пп.
  iter-2 уже убрал 3 обхода → 1; остаточная цена тика — per-goal мутации
  lockedFlags (EnumMap get/put/remove на КАЖДОМ stop/start) и промахи
  canUse-порога. Батч превращает N мутаций x K флагов в N append-ов в
  дифф-буфер (ThreadLocal, без аллокации после прогрева) + 1 проход
  применения и 1 пересчёт флагов: на моба с 6-8 goals и 1-3 переходами/тик
  экономия ~40-60% ветки переходов, ветка = доля единиц pp goalops-хвоста →
  Δ +0.3-0.6пп (прогноз карточки, подтверждается разбивкой).
- Прогноз в финал: Δ +0.3-0.6пп (goalops-хвост, monotone закон-8: не
  ухудшает чёт/нечёт parity — гейт-оракул ниже).

## Источники (верифицированы HTTP 200, 2026-09-26)

1. https://docs.papermc.io/paper/dev/mob-goals — Paper Mob Goal API:
   жизненный цикл goal-объекта (stop/start семантика, порядок активации по
   приоритетам) — контракт insertion-order для батча.
2. https://github.com/CaffeineMC/lithium-fabric/wiki/Configuration-File —
   lithium `mixin.ai.goal`: «a faster implementation of the goal selector
   for entity AI» — прецедент канона оптимизации GoalSelector без
   изменения наблюдаемого порядка (батчинг/флэт-структуры).
3. https://nekoyue.github.io/ForgeJavaDocs-NG/javadoc/1.18.2/net/minecraft/world/entity/ai/goal/GoalSelector.html
   — javadoc GoalSelector: `tickRunningGoals(boolean)`, `tickCount`,
   разделение фаз — парити-грунд для чёт/нечёт сплита и stop/start диффа.

## NCDFE-канон (обязателен)

- Мост GoalOps define+init в kernel loader ДО флипа READY (нельзя
  ретаргетить сайт на статик несуществующего класса → NoClassDefFoundError
  в server tick); fail-inert: define failed → хук остаётся спящим.
- Блоб-гейт: rust-сторона сканирует include_bytes! маркеры
  `transitionDiffGate` / `transitionDiffRunningGate` / `cmp459_p47` — нет
  маркеров = сайты НЕ трогаются (блоб старый → rebuild первым шагом ноги).
- Гейт STRICT-eq `cmp459_p47` (пустой/чужой флаг = ваниль бит-в-байт).
  Разделяемость с iter-2: cmp459_p47 НЕ входит в STRICT-OR список
  goal_selector.rs → двойное define/retransform одного класса невозможно
  (гейты взаимно исключающие по env).
- Class-major guard: блоб --release 21 (major 65) ≤ JVM major, иначе
  dormant (урок 408).

## Пререгистрированные гейты вердикта

1. cargo check --lib 0 err; cargo test --lib зелёный (новые тесты диффа:
   insertion-order, 1 пересчёт/тик, capacity-guard, blob-маркеры, дискрет
   гейтов).
2. CI run id (workflow_dispatch на ветке; 422 → фолбэк p500-smoke/bench).
3. Виринг: WILD-верификация = файлы+LOC+тесты+run id (12e), не измерение
   mspt (нога-измерение — следующий шаг канона после CI-зелёности).
