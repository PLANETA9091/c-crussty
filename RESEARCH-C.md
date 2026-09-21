# RESEARCH-C — TASK-401-C: замена nav-подсистемы с индексом (cmp401_navsys)

Дата: 2026-09-21 14:3x +08. Веткa: round-401-c-navsys (base d26f524 = origin/master).
Вектор: nav_ai ~14.7% wall, рассыпан (death-by-cuts) → ЗАМЕНА ПОДСИСТЕМЫ
по образцу items_index (items-лейн 31.17% → 0.00% java на ветке round-398-j-subsys2).

## 1. Профиль-декомпозиция (recon-c-navtax.py, jmob2 @ cpu-collapsed 114439 сэмплов)

nav-attributed = 16815 = **14.69%**:
- `net/minecraft/world/entity/ai/goal` — **8.13%** (GoalSelector.tick self 0.51%,
  tickRunningGoals 0.46%, ObjectLinkedOpenHashSet$SetIterator.next 0.78%,
  goal-driven getEntities ChunkEntitySlices.getEntities 1.66% + EntityLookup 0.30%,
  RemoveBlockGoal.isValidTarget 0.18%, EnumMap.get 0.11%, Goal.getFlags 0.10%...)
- `net/minecraft/world/level/pathfinder` — **≈3.9%** (WalkNodeEvaluator 2.39%,
  Path 0.62%, PathTypeCache 0.43%, NodeEvaluator 0.29%, PathfindingContext 0.11%,
  Node 0.08%, PathFinder 0.07%, BinaryHeap 0.07%)
  - листья под WalkNodeEvaluator: PalettedContainer.get 0.50%, LevelChunk.getBlockStateFinal
    0.37%, PathTypeCache.get 0.31%, Long2ObjectOpenHashMap.find 0.18%,
    readPalette 0.13%, SimpleBitStorage.get 0.12%
- navigation 0.79% (shouldRecomputePath 0.24%), behavior 0.29%, Brain 0.03%,
  Profiler.get 0.24% (в serverAiStep-стеках).

Kernel-факты (javap patched-kernel.jar):
- `Mob.serverAiStep`: 2-фазный goal-гейт `(tickCount+getId())%2` (vanilla 1.21.2+),
  navigation.tick КАЖДЫЙ тик вне гейта; Profiler.get/push/pop на каждую секцию.
- `GoalSelector`: Moonrise уже поставил ObjectLinkedOpenHashSet + OptimizedSmallEnumSet
  (Lithium collections/goals уже в кернеле — этот рычаг занят).
- `PathNavigation.tick()`: при path==null и !hasDelayedRecomputation — фактически no-op
  (tick++, isDone, return) → inactive_navigations-порт даёт ~0.1-0.3%, НЕ масса.

## 2. Апстрим-пруфы (скачаны в research/lithium-proof/, ветка 1.21.1 CaffeineMC/lithium-fabric)

1. `ai/pathing/WalkNodeEvaluatorMixin.java` + `common/ai/pathing/PathNodeCache.java`:
   **кэш PathType на канонический BlockState** (`((BlockStatePathingCache)state).lithium$getPathNodeType()`)
   + section-level palette-скан (`isSectionSafeAsNeighbor`). Комментарий JellySquid:
   "Determining the type of node offered by a block state is a very slow operation
   due to the nasty chain of tag, instanceof, and block property checks. Since each
   blockstate can only map to one type of node, we can create a cache which stores
   the result of this complicated code path. This provides a significant speed-up
   in path-finding code and should be relatively safe."
2. `mixin/collections/goals/GoalSelectorMixin.java` — ObjectLinkedOpenHashSet для
   availableGoals (доказывает, что лейн GoalSelector — целевой для оптимизаций;
   в кернеле УЖЕ есть, вектор закрыт).
3. `mixin/ai/non_poi_block_search/RemoveBlockGoalMixin.java` — кэширование
   блок-сканов цели (RemoveBlockGoal.isValidTarget 0.18% в нашем профиле).
4. `mixin/ai/task/memory_change_counting/BrainMixin.java` + `common/ai/MemoryModificationCounter.java`
   — skip behavior-тик при неизменных памятьх (Brain/behavior 0.32% — не масса v1).
5. `entity/inactive_navigations/*` — skip navigation.tick для неактивных навигаций
   (в нашем кернеле no-op путь уже дешёв — декомпозирован выше, v2-кандидат).

Собственная техника (выбор): ржавый плоский индекс «канонический BlockState → PathType»
ключом `Block.getId(state)`-эквивалент (vanilla `BLOCK_STATE_REGISTRY` dense id, есть в
кернеле — сетевая сериализация). Блок-стейты иммутабельны-каноничны ⇒ кэш без
инвалидации ⇒ нулевой parity-риск (в отличие от pos-ключённых мемо — запрещены).
Rust-сторона = items_index-паттерн: открытая адресация long→byte, natives на
bridge-классе NavOps, RegisterNatives (прецедент batch_api/items_manager).

## 3. План v1 (subsystem step 1)

- `src/nav_path_type.rs`: flat table key=mix64(stateId)→byte(pathType ordinal), fail-closed.
- Java `entityinside/net/minecraft/world/entity/ai/NavOps.java` (аналог ItemEntityManager):
  static natives + `pathTypeCached(int stateId)` + арм-маркеры в stdout.
- Байт-патч `WalkNodeEvaluator.getPathTypeFromState(BlockGetter,BlockPos)` — fast-path
  через NavOps (после getBlockState-эквивалента; структура по javap ниже).
- ARM: CRUSSTY_LEVER_FLAG=cmp401_navsys; fail-closed: любой не-OK → ванильный путь.
- Метрика успеха итерации-1: nav lane 14.7% → ожидание −2..−4пп (pathfinder-часть),
  pair-by-runner vs same-day якоря; далее цикл (v2: goal-driven entity scans / inactive_nav).

## 4. Ограничения/запреты

Не трогаем: ZGC/alloc_diet/zero_alloc/flat_traversal/fluid_dirty-мемо/inside_bitmask/
fluid_bitmask/THP/RECON-42/players-16/GC-лотерея. Позиционные мемо PathType с
инвалидацией — запрещены по parity (block-update staleness) — потому канонический
state-ключ, а не pos-ключ.
