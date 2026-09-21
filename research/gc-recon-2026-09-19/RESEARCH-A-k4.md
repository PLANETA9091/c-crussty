# RESEARCH-A-k4 (TASK-409-A): nav_ai остаточная декомпозиция после aleg2 + вердикт об исчерпании bulk-поверхности

Контекст: navplane серия (cmp405_navplane): aleg1 2.3@6507187 pair +9.5pp ARMED ×7,
aleg2 2.6@7237353 pair +10.6pp ARMED (absorb tick-408), aleg3 = min-of-3 (run 35667055638).
Лейн nav_ai на aleg2: 13.92% wall (15808/113552 сэмплов) — против 14.16% базлайна
(−0.24пп флэт: окно sendBlockUpdated уже съело свой срез в aleg1, −0.9пп).

## Свежая декомпозиция nav_ai (aleg2 cpu-collapsed, round-round405aleg2)

| subtree | % lane | % wall | природа поверхности |
|---|---|---|---|
| GoalSelector.tick (canUse→findTarget→getEntities + flag-циклы + итератор) | 52.1 | 7.25 | canUse = RNG side-effect (RECON-8 PARK); findTarget/getEntities = BROADPHASE-лейн (eindex, агент C); flag-циклы (goalContainsAnyFlags 0.52, tickRunningGoals 1.06, goalCanBeReplaced 0.30) = Java-структура, решения per-mob same-tick |
| PathFinder.findPath (A* под createPath) | 21.9 | 3.05 | доминируют world-queries: getBlockState/getFluidState/getBlockStateIfLoaded/isStableDestination — чтения блоков на РАСШИРЕНИЕ узла (per-node) = мост-антипаттерн (fluidplane RED, 16.72→71.48%); node-пул/HIP-heap = пулаемый хвост <40% |
| tick-head (PathNavigation.tick subtree: followThePath 0.09, isDone 0.14+0.07, doStuck 0.05, getGroundY 0.13) | ~3 | 0.64 | решения per-mob same-tick-consumed (доказательство ниже) → per-entity JNI = дизайн-ошибка (закон 6) |

## ПАРИТИ-ЛОВУШКА tick-head (доказательство невозможности deferred-батча)

javap PathNavigation.tick (kernel @d538a32, purpur-1.21.10):
1. tail (offset 139-185): `setWantedPosition(getNextEntityPos(mob), getGroundY(vec), speed)` —
   getNextEntityPos читает node[nextNodeIndex] ПОСЛЕ advance из followThePath → хвост
   потребляет решение прибытия того же тика;
2. MoveControl.tick выполняется в том же serverAiStep СРАЗУ после navigation.tick
   (order: sensing → goalSelector.tick → navigation.tick → moveControl.tick → customServerAiStep)
   → wantedPosition потребляется тем же тиком;
3. customServerAiStep/Brain-бихевиоры того же моба читают isDone/getPath после nav.tick.
Следствие: любое deferred-применение (end-of-loop flush) сдвигает решение на ≥1 тик =
сдвиг wantedPosition/observed isDone = нарушение бит-в-бит. А same-point решение =
per-entity JNI (закон 6 запрещает). Позиции моба, прочитанные в nav-tick тика T,
не существуют нигде раньше (travel/интеграция позиции — ПОЗЖЕ aiStep того же моба,
значит «на конец предыдущего тика» = другая точка = другие входы) → input-identity
re-validation всегда фейл → всегда ваниль → JNI-ноль. Батч-поверхность tick-head = ∅.

## Внешние пруфы (веб-поиск, 3+)

1. **Pufferfish DAB** (docs.pufferfish.host, pufferfish.yml): «DAB is an optimization that
   reduces the frequency of brain ticks. DAB does impact the AI goal selector behavior of
   all entities» — самоопределение антипаттерна: снижение частоты AI-тиков МЕНЯЕТ поведение.
   Наш PARK по GoalSelector/Brain (RNG side-effect canUse) подтверждён экосистемой.
   Гайды (diekieboy docs): «Throttles the AI goal selector … every 20 ticks instead of» —
   все внешние lever'ы этого класса = behavior-changing → вне parity-бара проекта.
2. **Lithium goal-selector** (github CaffeineMC/lithium issue #743): оптимизация =
   «custom list that allows skipping inactive elements» + cooldown-структуры — т.е.
   оптимизация СТРУКТУРЫ контейнера (итератор ObjectLinkedOpenHashSet.next = 0.65% wall
   у нас), без пропуска canUse-вызовов. Эквивалентная нашим данным структура-лейн, не
   decision-batch: Rust-bulk-форма отсутствует (структура живёт в Java-поле GoalSelector).
3. **Lithium ai.pathing** (curseforge/modrinth/jellysquid.me): «optimizes mob AI pathfinding»
   = node-pool/кэш path-node-types/heap-микрооптимизации ВНУТРИ A* — техника-прецедент
   для нашего findPath 3.05%, но у нас узел A* = world-query на расширение (getBlockState
   per-node) → per-node JNI = мост-антипаттерн (fluidplane RED 16.72→71.48% урок);
   пулаемый хвост (Node-аллокации) <40% от 3.05% ≈ 1.2% — кандидат k5, не k4.

## Вердикт k4

nav_ai 13.92% ДЕКОМПОЗИРОВАН ПОЛНОСТЬЮ: 7.25% = RNG/broadphase/структура (PARK/eindex),
3.05% = world-query A* (мост-антипаттерн PARK; k5-кандидат = node-pool ~1.2% потолок),
0.64% = tick-head (пари-ловушка, per-entity JNI запрещён, ∅-поверхность).
**nav→Rust bulk-поверхность ИСЧЕРПАНА** законом (6) + парити-ловушкой. Окно
sendBlockUpdated (aleg серия) = последняя легальная bulk-форма лейна; min-of-3
закрывает вектор. k5-цель (NEXT-410): A* node-pool (Lithium-прецедент, ~1.2% потолок,
требует мульти-сайт ретаргет PathFinder/PathNodeNavigator — оценка ≥1 полный тик)
и хэндофф findTarget/getEntities → eindex (агент C, cleg-серия).

ВАНИЛЬНОСТЬ: без изменений кода — aleg3 = тот же @e88f73b (подтверждение min-of-3
по RESEARCH-A-k3: «если aleg2 ≥80% от aleg1 — рисёрч-итерация не требуется»).
