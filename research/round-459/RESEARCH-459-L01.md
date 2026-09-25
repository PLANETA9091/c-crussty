# RESEARCH-459-L01 — inside_blocks / checkInsideBlocks (TOP-1 остаток 12-16.6%), capture-матем для ID-P31 INSIDE-BATCH bulk-JNI

Агент: TASK-459-L01, v18.3 законы 13-16. Носитель-цель: cmp456_chunkmono @d73758a (моно-семья, MONSTER chk-14 +21.7@8687055).
Климб-миссия (закон 13c): пара ≥+20 поверх chk-14; P31 INSIDE-BATCH bulk-JNI (candidate-superset → strict java tail).

---

## 1. Профили-основания (сэмплы, wall/CPU-collapsed, ABSORB)

| нога | runner | norm | CPU total | inside_volatile (стек) | inside абс. сэмплов | java_util | вердикт |
|---|---|---|---|---|---|---|---|
| chkmono457-11 (депресс) | 7040413 | −3.6% | 104163 | **16.63%** | **17322** | 8.96% | RED |
| chkmono457-12 (депресс) | 7053858 | −3.7% | 104260 | **16.30%** | 16994 | 8.73% | RED |
| chkmono457-14 (МОНСТР) | 8687055 | **+21.7%** | 103062 | **15.36%** | 15830 | 8.26% | GREEN-CAND |
| chkmono457-16 | 6951662 | +12.7% | 104735 | **16.45%** | 17229 | 8.59% | GREEN-CAND |
| anchor458-33 (сноска эры) | 6973621 | −11.8% | 115503 | 10.47% | 12093 | 6.66% | RED |

Базлайн банка: 12.01% × 115655 = 13885. ФАКТ: на ногах носителя лейн РОС АБСОЛЮТНО: 15830 (+14% к базлайну), 16994-17322 (+22-25%). На свежем якоре 12093 (−13%). Разница нога-минус-якорь: +3737 сэмплов (chk-14) до +5229 (chk-11) — это не композиционный эффект, это ДОБАВЛЕННАЯ работа inside-плоскости на носителе (гипотеза B: invalidation-driven, см. §4).

Лист-ранжирование CPU-листьев внутри лейна (BOTTLENECKS_3, точные сэмплы):

| leaf | chk-11 | chk-14 | chk-16 | anchor33 |
|---|---|---|---|---|
| InsideSnapOps.serve4 | 2012 (1.9%) | 2303 (2.2%) | 1947 (1.9%) | **0 (нет)** |
| InsideBlockOps.gate | 1183 (1.1%) | 1125 (1.1%) | 973 (0.9%) | 1309 (1.1%) |
| VarHandleRefs$FieldInstanceReadOnly.getVolatile | 1146 | 1359 | 1488 | 1867 |
| ConcurrentHashMap.get | 982 | 992 | 951 | нет в top-40 |
| LongOpenHashSet.add (visitedBlocks) | 1029 | 999 | 918 | 1134 |
| Entity.applyEffectsFromBlocks | 921 | 859 | 828 | 961 |
| PalettedContainer.get (общий) | 4179 | 4151 | 3952 | 5384 |

Wall-стек-декомпозиция внутри лейна (обе GREEN-ноги + якорь, доля стеков checkInsideBlocks):
- 100% стеков проходят через InsideBitmaskOps.checkInsideBlocksGated (обёртка носителя), 95-96.5% через MethodHandle ORIG.invokeExact (LambdaForm$MH.invokeExact_MT + DMH.invokeSpecial — 3 некинлируемых кадра на КАЖДЫЙ вызов);
- serve4/snapGet: 21-32% стеков (носитель) / 0% (якорь);
- caller 100%: Entity.applyEffectsFromBlocks → checkInsideBlocks; доминирующие owner-пути: ItemEntityManager.tickBody (item×106459 фикстуры) и LivingEntity.aiStep (Zombie.aiStep и пр.);
- RegionTickOps.tickBucket: 100% стеков (region-worker потоки);
- aabb-математика (deflate/inflate/intersects/distanceToSqr): 12.8-23.7% стеков лейна;
- entityInside+getEntityInsideCollisionShape: 2.1-4.9%; effectApplier: 0.4-0.5%; fluid-рид: 6.3-11.6%;
- broadphase-discovery ВНУТРИ лейна: 0.0% (discovery тут не entity-broadphase, а BlockPos-итерация — см. javap).

## 2. javap-контракты (7, канон round-396-a jar + carrier build agent-i)

C1. `Entity.checkInsideBlocks(List<Entity$Movement>, InsideBlockEffectApplier$StepBasedCollector)`:
бюджет `bipush 16` на свип; `visitedBlocks:LongSet` (fastutil, clear в конце); для каждого Movement: from/to, `Vec3.subtract`, `Direction.axisStepOrder` (ImmutableList-итератор) → осевые свипы `checkInsideBlocks(Vec3,Vec3,collector,LongSet,I)`; фолбэк: полный свип + добивка `checkInsideBlocks(to,to,...,1)`.
C2. `Entity.checkInsideBlocks(Vec3,Vec3,...,int)` (visit-ядро): `makeBoundingBox` → `AABB.deflate(9.9999997E-6)` → `Vec3.distanceToSqr` vs `Mth.square` → `BlockGetter.forEachBlockIntersectedBetween(from,to,aabb,BlockStepVisitor)` (lambda$checkInsideBlocks$2), AtomicInteger-счётчик.
C3. `lambda$checkInsideBlocks$2` (VISIT на блок): `level.getBlockState` → `BlockState.isAir` (skip) → `getEntityInsideCollisionShape` → `VoxelShape.move` → **`VoxelShape.toAabbs()` (аллокация List<AABB> на каждый не-air блок!)** → `collidedWithShapeMovingFrom` (свип) → `getFluidState` → `collidedWithFluid` → `BlockPos.asLong` + `LongSet.add` → `AABB.intersects(BlockPos)` → `StepBasedCollector.advanceStep(I,BlockPos)` → `BlockState.entityInside(...)` → `onInsideBlock` → `FluidState.entityInside(...)`; debug-ветка (AtomicInteger, debugSynchronizers) под гвардией.
C4. `Entity.applyEffectsFromBlocks(List<Movement>)` (TAIL): `isAffectedByBlocks()` гейт → stepOn → `checkInsideBlocks(list, insideEffectCollector)` → `collector.applyAndClear(this)` — строгий java-хвост эффектов.
C5. `InsideBlockOps` (lever #3 inside_cache, АКТИВЕН на банке): flat `SLOT_EID/FX/FY/FZ:long[]`, `VIS_POS/VIS_STATE`, `EFF_POS/EFF_STATE/EFF_STEP/EFF_FLAG`, `NSLOTS/MAXVIS/MAXSTEPS`, `volatile SNAP_ARMED`, Unsafe COL_OFFSET, `gate(...)` = ARMED-гейт + slot-мемо + `mirror(...)`. → **70% инфраструктуры P32 candidate-superset уже существует в flat-массивах.**
C6. `InsideSnapOps` (SNAP sidecar, есть в блобе, v4/serve4): `CHM<LevelChunkSection,Snap> SNAPS` (это CHM.get, кормящий java_util!), `volatile ARMED/V4`, flat `META/WBUF/OUT/SECB/PALB/SNAPB/GENB`, `STAT_HITS/MISSES/COLLECTS/SECTIONS/INVALIDATIONS`, per-thread `LANE_TL`, `serve4(Level,BlockPos)` = section→Snap→flat palette read, fallback `serve`→`level.getBlockState`. → P32+P36: флет-реестр + epoch fast-gate (GENB), демо CHM→flat.
C7. `InsideBitmaskOps` (обёртка #15, флаг OFF): `checkInsideBlocksGated` → `skipCandidate`?нет → `invokeOriginal` → `MethodHandle ORIG.invokeExact` — МЕТОД-ХЭНДЛ на горячем пути; `sweptHullInto/hullAllAir` (air-hull pre-gate) — вектор закона 5, ЗАПРЕЩЁН к активации (×73 паритет), используется только как справка по hull-математике.
C8. `CollideBatchOps` (паттерн-прецедент P31): `THRESH/DENSE_LIMIT/TABLE/POOL_CAP`, `buildPlan(LevelChunkSection,...)`/`walkPlan(...)`/`scanSectionVanilla(...)` + `BiPredicate<BlockState,BlockPos>` строгий хвост + KIND_DYNAMIC fallback (поршни) — канон candidate-superset → strict java tail.
C9. `InsideBitmaskLockstepHarness` — офлайн lockstep-оракул уже существует (G2-прецедент).

## 3. Декомпозиция лейна на фазы (числа, CPU-база chk-14 15830 сэмплов)

| фаза | состав | сэмплы | % CPU | % лейна | захват P31 |
|---|---|---|---|---|---|
| DISCOVERY | BlockPos-итерация forEachBlockIntersectedBetween, AABB.deflate/inflate/intersects, Vec3.get/choose/distanceToSqr, axisStepOrder | ~3.4k | ~3.3% | ~21% | ДА (Rust, bit-exact) |
| VISIT | getBlockState→PalettedContainer.get (inside-доля ≈ 1.0-1.4пп из 4.0пп общего), isAir, LongSet.add, VoxelShape.toAabbs+move, collidedWithShapeMovingFrom свип | ~6.1k | ~5.9% | ~38% | ДА для статик-шейпов (KIND_DYNAMIC → java) |
| CARRIER-OVERHEAD | serve4+snapGet 2.3k, gate 1.1k, CHM.get 1.0k, LongOpenHashSet.add 1.0k, MH-wrapper (некинлируемый, ~0.3-0.5k) | ~5.4k | ~5.3% | ~34% | ДА (batch-feed заменяет мемо целиком) |
| EFFECTS-TAIL (strict java) | applyEffectsFromBlocks 859, advanceStep/applyAndClear, entityInside/FluidState.entityInside, setTicksFrozen/setDeltaMovement/setOldPos (доля) | ~0.9-1.5k | ~0.9-1.5% | ~7% | НЕТ (по дизайну, парити мутаций) |

Контроль по якорю (вне носителя): lane 10.47% − gate 1.13% − tail ~0.83% ≈ **8.5пп ваниль-ядра (discovery+visit)** — совпадает с носителем (8.9пп) ±0.4 → декомпозиция консистентна на двух независимых конфигурациях.

## 4. Гипотеза роста лейна на RED-ногах (проверка числами)

Абс. внутри-сэмплы: якорь 12093 → chk-14 15830 (+31%) → chk-11/12 17322/16994 (+43%/+41%). При этом: serve4+gate+CHM ≈ 4.3-4.5k сэмплов на всех ногах носителя (флэт) — рост идёт НЕ в мемо-хвост, а в visit/discovery ядро → подтверждается гипотеза B: chunk6-sched setBlockState-всплески → secWrite-бампы → CHM<Section,Snap>-инвалидации (STAT_INVALIDATIONS) → serve4 MISS-путь (`serve`→`getBlockState` двойной ход) + LongSet-перестройки. java_util растёт синхронно (7.01→8.73-8.96%, CHM.getNode + HashMap.getNode 1.6%). P31 batch-feed ЛИКВИДИРУЕТ сам механизм (нет per-pos serve → нет MISS-домино), поэтому на депресс-ногах прогноз захвата ВЫШЕ среднего.

## 5. CAPTURE-МАТЕМ (базовая формула брифа п.5: Δ = lane% × захват%; потолок = lane%)

Лейн chk-14 = 15.36% (15830 сэмплов). Захватываемое ядро: discovery+visit (9.2пп) + carrier-overhead (5.3пп) = 14.5пп = 94.4% лейна; строгий java-хвост 0.9-1.5пп не захватывается → макс-захват = 14.5/15.36 = **94%** (реалистичный max ≈ 92% с поправкой на KIND_DYNAMIC/поршни и debug-гвардии).

Сценарии (эффективность захвата ядра / overhead):
- КОНСЕРВАТИВ 50% / 60%: Δ = 15.36 × (0.50×0.60 + 0.60×0.345) ≈ 15.36 × 0.507 ≈ **+7.8пп** → нога 21.7+7.8+2.0(P32+P36) = **31.5пп**; пара с a26 +12.4@8671791 (Δ=15264≤50k, pair-fresh) = **+19.1** ✗ (0.9 до бара; нужен якорь ≤+11.5 в окне B)
- БАЗА 65% / 80%: Δ = 15.36 × (0.65×0.60 + 0.80×0.345) = 15.36 × 0.666 ≈ **+10.2пп** → нога **33.9пп**; пара с a26 = **+21.5** ✓ БАР ВЫПОЛНЕН существующим якорем
- МАКС 80% / 90%: Δ = 15.36 × (0.80×0.60 + 0.90×0.345) = 15.36 × 0.790 ≈ **+12.1пп** → нога **35.8пп**; пара с a26 = **+23.4** ✓
- ПОТОЛОК (100% захват захватываемого, 92% лейна): Δ = **+14.1пп** → нога 37.8; пара с a26 = **+25.4**; абсолютный потолок лейна 15.36пп → пара +24.7 при P32=0.
- На депресс-ногах (лейн 16.63/16.30) захват тот же механизмом: Δ база ≈ 16.63×0.666 ≈ +11.1пп, но norm ног отрицательный (−3.6) — в пары не годятся, годится только как аргумент «механизм ловит MISS-домино».

Сводка-прогноз: **Δ(P31 на chk-14) = +7.8..+12.1пп (база +10.2), потолок +14.1пп; нога 31.5..35.8 (база 33.9); пара ≥+20 достижима при захвате ≥53% (Δ≥+7.0 при якоре a26)** — ниже консерватива почти не промахиваемся; требование к якорю при базовом захвате: ≤+13.7 (есть a26 +12.4 ✓; окно B [8637055,8737055], норма ≥30 якорных ролов).

Бюджет паритета (dirty-list мутантов обязателен): строгий java-хвост = StepBasedCollector.advanceStep/applyAndClear + entityInside/FluidState.entityInside только по flagged-кандидатам (EFF_POS/EFF_STATE/EFF_STEP/EFF_FLAG flat-контракт C5 уже существует) + KIND_DYNAMIC/поршневой фолбэк + visitedBlocks LongSet семантика (bit-в-байт оракул G2).

## 6. PREREGISTERED ГЕЙТЫ G1-G6

- **G1 ARM/эффект-маркеры**: InsideBatchOps.arm() → stdout-маркер + STAT-счётчики (batch_calls/batched_entities/tail_candidates/dynamic_fallbacks) в логе; после вайринга javap flat==nested 10/10 EQUAL + strings-проверка блобов (урок-408 ×2: спящие гейты = DELIVERY-FAIL).
- **G2 lockstep бит-в-байт оракул**: расширить InsideBitmaskLockstepHarness: N=200k случайных (Movement, AABB) на реальных секциях фикстуры; batch-VIS/EFF (POS/STATE/STEP/FLAG) == vanilla поразрядно; AIOOBE=0; selfTest==true (×1-2 FAIL = fixture-шум); ERR_STRUCT/ERR_RANGE=0.
- **G3 young/Full GC**: young ≤ 130, Full ≤ 9, total-pause ≤ 22.1s (банк-справка); батч-буферы ThreadLocal/flat → нулевой.alloc-прирост (alloc-lane дельта ≤ +0.3%).
- **G4 популяция-паритет**: pop 140-165k, entity-churn дельта ≤1.5пп, spawnable 289-стабилен, items 0.00-гейт на серт-ногах (28.7-30.8% ваниль на якорях).
- **G5 TPS-бар vs банк v4**: norm = median/TPS_exp(runner)−1; band 6.0-9.5M (ре-ролл ≤2); pair = leg_norm − anchor_norm ≥ +20 при Δrun ≤50k pair-fresh, min-of-3; NOT-A-BENCH маркер + argv-guard.
- **G6 fail-closed disarm**: любое нарушение G2/ERR_* → disarm() → ванильный invokeOriginal-путь; threw=0 обязателен; NCDFE=0 (EARLY-define канон: если появляется InsideBatchOps-класс — define в раннем arm-хуке).

## 7. Внешние источники (по памяти, ≥3: проект + тема + что взято)

1. **Moonrise (ca.spottedleaf)** — chunk system entity lookup + CollisionUtil section-walk (getHardCollidingEntities/EntityCollectionBySection в профилях); взято: buildPlan/walkPlan паттерн по LevelChunkSection-палитрам + LazyEntityCollisionContext-хвост (уже канонизирован в CollideBatchOps, C8).
2. **Lithium (CaffeineMC)** — кэш упрощённых/сдвинутых collision-шейпов (simplified shape cache, mixin на VoxelShape/VoxelShapes) против повторного toAabbs(); взято: pre-swept AABB superset для статик-шейпов в Rust-visit (устраняет toAabbs-аллокацию C3 на каждый не-air блок).
3. **C2ME** — параллельный тикинг, per-worker ThreadLocal scratch без контеншна; взято: THRESH-батчинг на region-worker bucket перед JNI-диспатчем (по одному bulk-вызову на батч, нулевой шаринг между воркерами).
4. **Paper PR InsideBlockEffectApplier / StepBasedCollector** — step-based батчинг эффектов внутри-блоков с порядком применения; взято: строгий java-хвост поверх advanceStep/applyAndClear (порядок эффектов = бит-в-байт паритет, G2).
5. **Pufferfish/krypton/noisium** — вне лейна (simulation-distance/network/noise) — релевантности внутри-плоскости нет (честный ноль, не натягиваю).

## 8. Вердикт и следующий шаг

Лейн жив: ваниль-ядро discovery+visit ≈ 9.2пп CPU на ногах носителя + 5.3пп carrier-overhead = 14.5пп захватываемого; strict-tail ~1пп остаётся. P31 INSIDE-BATCH по паттерну CollideBatchOps (C8) + flat-контракт InsideBlockOps (C5): прогноз Δ **+10.2пп база (+7.8..+12.1 коридор), потолок +14.1пп**; нога chk-14 → **33.9пп (31.5..35.8)**; пара с существующим a26 +12.4 = **+21.5 база** ✓ (консерватив +19.1 — добор якорем ≤+11.5). П32+P36 (+1.5-2.5пп, 70% кода существует в C5/C6) — обязательный компаньон для страховки бара.
СЛЕДУЮЩИЙ ШАГ (конкретика): имплементация InsideBatchOps (THRESH=512/bucket, buildPlan-порт C8, EFF-флэт-выход C5) + lockstep-харнесс G2 офлайн → CI-нога на носителе d73758a с парой-окном B. Не активировать inside_bitmask/hull-гейт (закон 5, ×73) — только batch-feed поверх invokeOriginal-замены.
