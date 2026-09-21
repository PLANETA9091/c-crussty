# RESEARCH-A-k3 (TASK-405-A рестарт-2): nav_ai после leg1 — остаточная декомпозиция и aleg3-кандидаты

Контекст: aleg1 (run 35649236693) GREEN-CANDIDATE 2.3@6507187, pair +9.5pp vs 2.10@6845027,
но nav_ailane съел только −0.9пп (14.16→13.28% из 112436 сэмплов). Срез leg1 = только
ServerLevel.sendBlockUpdated → navigatingMobs-итерация → shouldRecomputePath (Vec3-alloc/моб/апдейт удалён).

## Остаточный nav_ai (leg1 cpu-collapsed, lane 14927 сэмплов, топ-хвосты)

| subtree | samples | % lane | % wall | поверхность |
|---|---|---|---|---|
| getEntities под Brain/behavior (Sensing-сканы) | ~3050 | 20% | 2.7% | BROADPHASE-лейн (агент C, eindex) — не наш |
| GoalSelector.tick + tickRunningGoals | ~3000 | 20% | 2.7% | canUse/canContinueToUse = RNG side-effect — RECON-8 PARK |
| — из них голый итератор ObjectLinkedOpenHashSet.next | ~1100 | 7% | 1.0% | машина итерации, отдельно от RNG не вырезается |
| — WrappedGoal.getFlags | ~440 | 3% | 0.4% | тривиальный геттер, вызовов слишком много — batch без меток невозможен |
| — Profiler.get miss (ThreadLocal) | ~317 | 2% | 0.3% | профайлер-лейн, cross-lane |
| PathTypeCache.getOrCompute (+fastutil find) | ~560 | 4% | 0.5% | createPath A* world-query — RECON-8 PARK |
| PathNavigation.tick subtree (isDone/followThePath/tick-хвост) | ~650 | 4% | 0.6% | ЕДИНСТВЕННЫЙ чистый RNG-free хвост нашего лейна |
| AttributeMap.getValue | ~300 | 2% | 0.3% | attributes-лейн, не наш |

## javap PathNavigation.tick (kernel @ca5e1a4, round-406-anchora jar) — структура

1. `this.tick++` (int counter)
2. `hasDelayedRecomputation` → `recomputePath()` (состояние-машина, флаг ставится stop()/shouldRecomputePath-путь)
3. `isDone()` → return (Path.isDone = getNextNodeIndex >= getNodeCount)
4. `canUpdatePath()` (Mob.onGround || isInLiquid...) → `followThePath()`
5. else: `!path.isDone && tempMobPos.y > next.y && floor(x)==floor(next.x) && floor(z)==floor(next.z)` → `path.advance()` (застревание под y)
6. хвост: `!isDone` → `getNextEntityPos(mob)` (Vec3-alloc!) → `moveControl.setWantedPosition(x, getGroundY(vec), z, speed)` — **getGroundY = Level.getBlockState = world-query, остаётся ванильным**

## javap followThePath — математика (RNG-free, pure-decision)

- `maxDistanceToWaypoint = bbWidth > 0.75f ? bbWidth : 0.75f - bbWidth` (fcmpl ветвление)
- d-разности до getNextNodePos: `abs(mobX - (nx+0.5))`, `abs(mobY - ny)`, `abs(mobZ - (nz+0.5))` — i2d/dadd/dsub/Math.abs
- прибытие: `dx < maxD && dz < maxD && dy < 1.0` (dcmpg ×3)
- далее: canCutCorner(next.type) → цикл пропуска узлов с `Vec3.distanceToSqr` — **уже ПОКРЫТ degenerate-входами, но это цикл по path-массиву**
- мутации: `path.setNextNodeIndex(i)`, `getTempMobPos` Vec3-alloc ×2/тик (tick-head + followThePath + tick-tail getNextEntityPos = 3 Vec3/нав-моб/тик)

## Вердикт по углублению (если aleg2 <80%)

Кандидат k3 = "nav-tick head": батч-решение пунктов 2-5 (isDone/advance-decision/застревание)
на ВСЁ множество navigating мобов ОДНИМ JNI, мутации (recomputePath/advance/followThePath)
в Java в порядке тика; tick-хвост setWantedPosition+getGroundY НЕ трогаем (world-query).
Экономика: PathNavigation.tick-subtree ~0.6% wall + 3 Vec3-alloc/нав-моб/тик (GC-производная).
Потолок дельты скромный: ~+1.5-3pp pair. GoalSelector/Brain/behavior = PARK подтверждён
(RNG side-effects / чужие лейны).

ВАНИЛЬНОСТЬ: пустой/чужой флаг = ваниль (тот же STRICT-гейт); armed-математика javap-verbatim
(fcmpl/dcmpg контракты, Mth.floor = (int)floor для >=0, floor-сравнения через i2d);
fail-closed → java-реплика. Если aleg2 ≥80% — aleg3 = подтверждение min-of-3 тем же кодом
(рисёрч-итерация не требуется по циклу закона).
