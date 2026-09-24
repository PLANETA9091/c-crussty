# RESEARCH-451-D — sense/brain подсистема: ценз числами (канон wgen-×445: числами, не гипотезами)

Task ID: TASK-451-D · ветка round-451d-sense (master b5baf548) · 2026-09-24

## 0. Вопрос ценза

(a) Сколько от nav_ai 14.16% = GoalSelector/Brain/behavior/sensing vs PathNavigation (pathfinder — чужой вектор)?
(b) Что остаётся от sense/brain-среза на ТЕКУЩЕМ master (без ins4) — потолок вектора?
(c) Потолок < ~10% TPS-эквивалента → LOW-POTENTIAL ДО имплементации → план Б.

## 1. Источники (ретейн-артефакты + история эффектов)

- **wall-collapsed.txt round-round-449-anchor-2** (ваниль-якорь эры, 61259 wall-сэмплов, полный стек) + **cpu-таблица BOTTLENECKS_3.md того же рана** (117044 self-сэмплов) — единственный полный ретейн на текущей эре.
- **wall-collapsed.txt round-round-443g-ins4-1** (нога golden-449 на носителе round-436-b-ins6 @0707800, nav_ai 14.16→3.57 CPU-лейн) — ретейн остаточного профиля под ins4-носителем.
- lane_map банк v4 (absorb_round.py, 115655 cpu-сэмплов): nav_ai 16380 = 14.16%, rx `PathNavigation|GoalSelector|\.Brain|behavior|PathFinder`.
- Исторические lever-эффекты (CPU-лейн, банк v4 база): GSEL −8.2пп ×7 ног; bq/queryplane −4.9пп; aibatch-окно −5.4пп; multi/SoA −9.8..−10.0пп; ins4d-носители −10.36..−10.60пп (рез. 3.57-3.81).
- javap-цены из sense-исследований эры: 28956bd2 (TASK-439-A2: sensing ~2.2% self + 2.7-3.0% goalquery, chokepoint getNearestEntity), 49d1a19e (TASK-443-A: tickEachRunningBehavior@0-51, getRunningBehaviors@0-141).

## 2. (a) Декомпозиция nav_ai 14.16% (wall-ценз 449-anchor-2, full-stack)

Внутри nav-стеков (592 wall-сэмпла lane'а, доля от total):

| подслайс | stack-contains | deepest-ai-leaf | CPU-лейн оценка |
|---|---|---|---|
| GoalSelector+goals | 85.8% | goal 64.5% | ~8.0-8.4 пп (GSEL-эффект) |
| PathNavigation (в т.ч. из goal.start) | 29.6% | navigation 28.0% | ~3.5-4.2 пп |
| PathFinder | 17.7% | — | (внутри navigation) |
| Brain+behavior | 3.0% | behavior 1.7% + Brain 0.3% | ~0.4 пп |
| targeting | 2.2% | 0.8% | ~0.3-0.5 пп (частично в goals) |
| sensing (ai/sensing+Sensor) | 0.2% | 0.2% | ~0.1-0.2 пп |
| move/look control | 1.0% | 0.8% | ~0.15 пп |

Cross-check CPU-таблицы (self-time): `GoalSelector.tick` self = 660/117044 = 0.56% total — self-ядро лупа; остальной лейн = калли-поддеревья (goals logic + queries + nav).

**Население фикстуры (BOTTLENECKS_3)**: item×103316, creeper/husk/skeleton/spider/zombie/drowned×~4.5-5.2k, sheep/cow/pig/chicken×3.2-3.5k — **Brain-мобы (piglin/hoglin/frog/allay/warden-класс) В ФИКСТУРЕ ≈ 0** → Brain/behavior-срез структурно микроскопичен на этой сцене (~0.4 пп).

## 3. (b) Что остаётся sense/brain-срезу на master — по-компонентно

| компонент | потолок лейна | статус (закрыт/покрыт/жив) |
|---|---|---|
| GoalSelector bookkeeping (луп, available, сортировки) | ~8.0-8.4 пп | **ЗАКРЫТ ×4 итерации / 9 ног (gsel)**: «lane-эффект РЕАЛЕН ×7, но стоимость батча > лейн-экономии на бокс-сцене СТРУКТУРНО» (GOAL ×416) |
| goal logic (canUse/start/tick целей) | ~5-6 пп | невыпиливаемо по закону 4 (решения ИИ = ванильное поведение) |
| goal-query/targeting | ~2.7-4.9 пп | **ПЛОСКОСТИ УЖЕ ЕСТЬ**: cmp417_bq в master (мерж ×417), eindexq+sense на носителях (3.1 пп); EQ-слой закрыт декомпозицией (TASK-446-EQ: pushEntities parity-wall S7-140 4.0-4.7 пп не наш) |
| Brain tick2 (tickEachRunning) | ~0.4 пп | жив, но микроскопичен (нет Brain-мобов в фикстуре) |
| sensing (Sensor) | ~0.1-0.2 пп | микроскопичен там же |
| PathNavigation+PathFinder | ~3.5-4.2 пп | **ЧУЖОЙ вектор** (pathfinder, не sense/brain) |

**Соло-потолок sense/brain**: оптимистично (весь GoalSelector-слайс при 100% конверсии) = 10.0-10.7 пп лейна ≈ +10-14% TPS теоретический максимум; реалистично (историческая конверсия body-swap семейства: sense-ноги +3.1..+6.9 norm, пары +4.2/−4.3; gsel ×4 закрыт структурно; bq уже в master) = **+2-4% TPS ≪ +20% бара и < ~10% критерия (в)**.

Конверсия лейн→TPS по эмпирике эры: ins4-монстры −45..−50 пп лейна → +19..+23% TPS (≈0.5/пп); mc3 −47 пп → +25-36%. Даже 10.7 пп при 0.5 конверсии ≈ +5%; при идеальной ≈ +11-14%. Оба < бара.

## 4. (c) ВЕРДИКТ: NO-GO соло-имплементации (LOW-POTENTIAL, честно, ДО кода)

Канон 445-C/wgen: потолок < ~10% TPS-эквивалента = не имплементим соло. Все 4 компонента слайса: gsel ЗАКРЫТ структурно, goal-query УЖЕ-ПОКРЫТ (bq в master), Brain+sensing МИКРО (фикстура без Brain-мобов), goal-logic НЕЛЬЗЯ по закону 4. Альтернативы R-меню тоже мертвы числами: POI 0.06%, despawn 0.34-0.38% (445-C), sscan2 RED −5.0 (2.10@6706212 — скан-оверхед > despawn-экономии, лейн-мишень 0.34%).

## 5. ПЛАН Б (закон 7): композиция cmp451_senseins на живом носителе

sense/brain целиком (SenseOps body-swap + BrainOps.tickEachRunning + sense-режим goalquery) поверх merge-носителя **round-436-b-ins6 @07078007** как STRICT-OR lever **cmp451_senseins** — прецедент sensemega @93c37845 (юнион 20 файлов, cargo 322/0, пары +4.2/−4.3, банк +16.8: «SOLO-sense мёртв, юнион жив»).

Ожидаемый добавленный эффект на носителе: остаточный nav 3.57 лейна (goal-leaf 51.8% + behavior 3.6%) → съедаемая доля ≈ 0.7-1.3 пп лейна ≈ **+1-3% TPS к носителю** — не самостоятельный вектор, а достройка семьи подсистем на merge-носителе (BOTTLENECK-451: «новый носитель или плотность»; закон 7: волна ≥2 R-вектора). Ноги — ПОСЛЕ серта ins4 (решётка главного агента); ветка round-451d-senseins = ins6 ⊕ sense-семья, ретаг cmp451_senseins на КАЖДОМ гейте (rust ~20 + java ~11), блобы, cargo, javap-гейты, STRICT-OR, пустой флаг = ваниль бит-в-байт.

## 6. Гаты решения

- [x] Ценз числами из ретейн-профилей и истории эффектов (не гипотезы)
- [x] Соло-потолок +2-4% реалистично / +10-14% теоретически < +20% бара
- [x] План Б выбран по промпту (ins6-носитель), альтернативы (POI/despawn) отсечены цензом
- [ ] Имплементация композиции + гейты + диспатч после серта ins4

## 7. Дополнение ценза (javap ground truth, kernel re-materialized paperclip pass)

Kernel jar восстановлен (purpur-1.21.10 paperclip pass → versions/1.21.10/purpur-1.21.10.jar, 29.4MB, ServerLevel/GoalSelector/Brain контент-тест OK) — использован для javac-гейтов блобов; javap-ценз структуры:

- `GoalSelector.tick()` = 119 бк-строк + `tickRunningGoals(boolean)` 32 + `goalCanBeReplacedForAllFlags` 39 — ядро goal-лупа (gsel-закрытый слайс)
- `Brain`: `startEachNonRunningBehavior` 63 / `getRunningBehaviors` 52 / `tickEachRunningBehavior` — Brain-срез существует, но Brain-мобов в фикстуре ≈ 0 → микро
- `PathNavigation`: `createPath` 166 + `doStuckDetection` 136 + `followThePath` 101 + `tick` 88 — pathfinder-стек = ЧУЖОЙ вектор (~3.5-4.2 пп лейна)
- javap подтверждает исторические цены 28956bd2/49d1a19e; декомпозиция §2-§3 остаётся в силе.

## 8. Реализация плана Б (выполнено)

- Ветка round-451d-senseins: master b5baf548 ⊕ round-436-b-ins6 @07078007 (66 файлов, --no-ff) ⊕ sense-семья (cherry-pick c6208a81 SenseOps + 01dcf2aa Brain-tick2; union-resolve 24 конфликт-блоков, STRICT-OR термины cmp438_sense сохранены)
- Ретаг cmp451_senseins на КАЖДОМ гейте: rust 18 файлов (mobs_sense/brainhook/entity_query/inside_snap v4/inside_bitmask/queryplane lever-map+mobs_manager 4/site-списки/colpush/chunk_parse/...), java 10 файлов (SenseOps.leverEnabled, BrainOps TICK2_FLAGS, EntityGoalQueryOps senseMode+FLAG_LABEL, ItemEntityManager ×3, MobScanOps, QueryPlaneOps, GoalOps, MobAiOps, MobPushOps ×2, ColpushOps FLAG6)
- Блобы: build_430b_blobs.sh (11 классов javac --release 21 против ре-materialized kernel) + BrainOps javac; flat==nested ×13 byte-identical; check_blobs_sync ALL IN SYNC (+cmp451_senseins нейдлы + NEW BrainOps block); javap-LOADABILITY gate_load ×13 OK (урок ×449)
- cargo: check --lib 0 err; test 314 passed / 0 failed (sense f2t2 + delivery tests на юнион-носителе)
- run_world3.sh: case cmp451_senseins (ARM-лог)
- Пустой флаг = ваниль бит-в-байт: все гейты STRICT-OR, регистрация идемпотентна, байты классов идентичны при чужом/пустом флаге
- Диспатч: scripts/dispatch_451d.py (argv-guard: только round-451d-*; dry-run; канон-inputs тика; golden_443.py НЕ используется)
- Ноги: ПОСЛЕ серта ins4 (решётка главного агента; per-ref concurrency позволяет параллельный ран, но приоритет решётке)
