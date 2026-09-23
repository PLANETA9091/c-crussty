# PROFILE-B2 (TASK-422-B brain iter-2, tick 10:08 +08, v17)

Носитель вектора: origin/round-421-mega 99ffefa (brain⊕chunk мега-носитель, мерж-кандидат тика).
Профиль среза: артефакт **round-421-mga** (run 35806660177, @99ffefa, lever=cmp421_brain ARMED,
3.20 TPS @ runner 8.82M) — это ТОЧНОЕ кодовое состояние базы ветки round-422-b-brain2,
поэтому он честнее для "остатка лейна", чем ванильный якорь-422 (у якоря sense-plane спит
и findTarget-сканы завышают brain-лейн). Дозагрузка round-422-anchorb — см. в конце.

## Лейн-карта mga421 (cpu-collapsed, TOTAL 109304 сэмплов)

| лейн | samples | % | примечание |
|---|---|---|---|
| fluid | 18153 | 16.61 | ЗАКРЫТ ×4 — не трогаем |
| broadphase | 14319 | 13.10 | colpush/queryplane носителя |
| inside_volatile | 13903 | 12.72 | агент A (cmp422_inside), НЕ мой срез |
| nav_ai (цель B) | 11916 | **10.90** | GoalSelector 9400 + PathNavigation/PathFinder 2516 |
| brain (цель B) | 1488 | 1.36 | Brain.tick/behavior/sensors |
| fastutil | 9062 | 8.29 | |
| paletted | 7022 | 6.42 | |
| items | 0 | 0.00 | ARMED-носитель гасит items (как предсказано) |

Целевой лейн B (остаток nav_ai/brain-family) = **~12.3%** (10.90 + 1.36).

## ГЛАВНАЯ НАХОДКА ПРОФИЛЯ: goal-selector fast-path НОСИТЕЛЯ — ПЛАЦЕБО

1. `grep GoalOps cpu-collapsed.txt` = **0 сэмплов**; `GoalSelector.tick` = 8220 (7.52%).
   Если бы GoalOps.tickGate работал — фреймов GoalSelector.tick не было бы вовсе.
2. server-stdout.log (строка 1186): `[crussty-plugin] goal_selector: GoalSelector.tick sites
   not rewritten (NotFound) — pass-through (fail-closed)` — ARM-маркер был, EFFECT-маркера
   `goal-selector EFFECT armed` в stdout НЕТ (grep = 0). Урок тика-409 подтверждён:
   ARM-строки не доказывают armed.
3. ROOT-CAUSE (javap + src/goal_selector.rs): retarget_virtual_to_static вызывался с
   ОКРУЖАЮЩИМ методом `tick()V` (FROM.1/FROM.2) вместо `serverAiStep()V`. В Mob.class ЕСТЬ
   свой `public void tick()V` (вызывает LivingEntity.tick + updateControlFlags) — скан
   шёл по ЕГО коду, где сайтов GoalSelector.tick нет → NotFound → fail-closed → ваниль.
   Сайты на самом деле в serverAiStep: javap purpur-1.21.10 `protected final void
   serverAiStep()V`: GoalSelector.tick @161 и @183 (чётные тики), GoalSelector.
   tickRunningGoals(Z) @113 и @136 (нечётные тики, Paper-сплит).
   → все "brain-ноги" тика-421 (+9.8…+16.3) и мега-ноги (+18.5/+9.3/0.0) несли только
   sense-plane (eqEpoch+senseArena, маркеры "epoch ok" ×2169 — РЕАЛЬНЫЙ арм);
   goal-часть cmp421_brain была инертна на ВСЕХ ногах.

## Разрез GoalSelector.tick (through = 9351 сэмплов, 8.56%)

| callee (drill-1) | samples | судьба при починке GoalOps |
|---|---|---|
| GoalStaggerOps.canUseGate | 3097 | остаётся (реальная работа canUse, stagger N=4 уже есть) |
| WrappedGoal.start | 1645 | остаётся (реальные start() → moveTo/createPath) |
| ObjectLinkedOpenHashSet$SetIterator.next | 1031 | **снимается** (плоский массив) |
| GoalSelector.goalContainsAnyFlags | 829 | **снимается** (inline hasCommonElements) |
| ObjectLinkedOpenHashSet.iterator | 374 | **снимается** |
| goalCanBeReplacedForAllFlags | 362 | остаётся (идентично ванили) |
| tickRunningGoals (внутр. pass-3) | 230 | **снимается** (плоский pass-3) |
| Profiler.get | 176 | снимается наполовину (hoist) |
| WrappedGoal.canContinueToUse | 126 | остаётся |
| leaf/self GoalSelector.tick | 1338 | частично |
| tickRunningGoals нечётные сайты (serverAiStep) | ~1131 | цель tickRunningGate (см. ниже) |

Ожидаемый съём плоского fast-path: ~2.5-3.0k сэмплов ≈ **2.3-2.7% CPU**.

## iter-2 слайс (выбор по профилю — ЗАКОН: слайс ЦЕЛИКОМ)

**GOAL-SELECTOR ПОДСИСТЕМА ЦЕЛИКОМ (починка + достройка, без natives):**
1. FIX: окружающий метод retarget = `serverAiStep()V` (не `tick()V`) — активирует
   спящий GoalOps.tickGate (флет-приоритеты, 1 обход + плоские проходы vs 3 ванильных).
2. EXTEND: ретаргет 2 сайтов `tickRunningGoals(Z)` (нечётные тики, ~1131 сэмпла) →
   новый `GoalOps.tickRunningGate(GoalSelector,Z)` — та же плоская семантика
   (isRunning && (stopAll || requiresUpdateEveryTick) → tick, порядок = insertion-order).
3. STRICT-OR гейт `cmp421_brain || cmp422_brain2` во всех 13 rust + 6 java гейтах
   (образец 40adceb/b6b1146) — ноги идут с lever=cmp422_brain2, носитель должен
   армиться ПОЛНОСТЬЮ.
4. Плюс brain-family хвост (Brain.tick 1.36%) НЕ трогаю: выборка мала, риск паритета
   не окупается (решение по профилю; Sensor/memory-срезы — кандидаты следующего
   витка цикла-3, если Δ<+20%).

Бан-лист уважен: natives не добавляются (goal-слайц = pure-java мост + byte-retarget),
flat_traversal/alloc_diet/memo не вводятся, гейт STRICT (пустой флаг = ваниль).

## Ожидания
- Починка активирует спящую часть носителя: ожидаемый дельта-бонус к меге ~+2.5-3.5% CPU.
- pair-вердикт — против свежих якорей-422 @57a2e67 (ids в DISPATCH.txt), медиана 3 ног.

## Дозагрузка якоря-422
- round-422-anchorb (35809267976) на момент среза ещё in_progress (диспатч 02:10:06Z).
- anchora/anchorc = band fast-fail (11.5M/10.8M), ре-роллы anchorar/anchorcr в полёте —
  вердикт-парение будет по ре-роллам/anchorb по закону pair-fresh.
- Файл будет дополнен, если anchorb успеет завершиться до диспатча ног.
