# AG-254 w526 — dp50k ItemEntity.tick суб-аттрибуция (0-POST, offline re-harvest)

Вилка OPEN «dp50k ItemEntity 20-21% CPU = таргет-1 S#3». Слоты dp50k 6/6 полны — POST-запрет
соблюдён (0 POST). Данные: re-harvest artifact-zip терминальных dp50k-ног (AG-16 пул, /tmp протух):
- AG-22a run 36971367106 @89a02a05, cpu_idx 7.398M, cpu-collapsed 80,426 сэмплов
- AG-37b run 36971305525 @240b1690, cpu_idx 7.366M, cpu-collapsed 85,372 сэмплов
Оба: pop50000 dp3v2 s42 fp4/gc3/ic1/fd1/rt4/bc1/r640/300s/xmx10G, world afb3a0b3, band 6.0-7.5M ✓
(верифицировано по run-env.txt из зипа, не по памяти).

## Сплит ItemEntity.tick (% ОТ ОБЩЕГО CPU-профиля ноги; async-profiler collapsed)

| суб-путь | AG-22a | AG-37b |
|---|---|---|
| **item-fluid-скан суммарно** (inWater+direct+eyes+fluid-shapes) | **7.33%** | **8.09%** |
|   — baseTick→updateInWaterStateAndDoFluidPushing | 4.66% | 5.17% |
|   — прямые fluid-вызовы из tick (getFlow и др.) | 1.80% | 1.76% |
|   — baseTick→updateFluidOnEyes | 0.72% | 0.99% |
|   — noCollision→LiquidBlock fluid-shapes | 0.19% | 0.15% |
| **checkInsideBlocks** (внутри applyEffectsFromBlocks, ItemEntity.tick) | **4.35%** | **4.51%** |
|   — applyEffects-other (эффекты без inside-скана) | 0.94% | 1.39% |
| **move/collide** | 3.36% | 3.27% |
| **noCollision-direct block-shapes** | 1.87% | 2.04% |
| data-get (getItem→SynchedEntityData.get) | 0.45% | 0.82% |
| applyGravity | 0.16% | 0.20% |
| SELF ItemEntity.tick | 0.20% | 0.15% |
| **mergeWithNeighbours** | **0.01%** | **0.01%** |
| Итого ItemEntity.tick | 19.41% | 20.99% |

Лист-хотспоты: PalettedContainer.get 1.40-1.54%, updateFluidHeightAndDoFluidPushing 1.27-1.66%,
CollisionUtil.getCollisionsForBlocksOrWorldBorder 0.60-0.68%, setDeltaMovement 0.58% (22a),
Mth.floor 0.46-0.47%, InsideBlockOps.gate 0.36-0.52% (armed-гейт жив).
Alloc (ItemEntity.tick): AABB_[i] 5.1-5.6% + Vec3_[i] 4.5-5.1% профиля аллокаций — физика боксов.

## Вердикты по prereg-гипотезам (claims/AG-254.md)
- **H1 (merge ≥5%) REFUTED ×2**: mergeWithNeighbours 0.01% на обеих ногах — канон Л145
  («ItemEntity.merge ≈0») подтверждён и на dp50k; merge-лейн НЕ воскрешать.
- **H2 (move ≥8%) REFUTED**: move/collide+block-shapes = 5.13-5.31% — третий таргет, не первый.
- **H3 (pickup ≥4%) REFUTED**: playerTouch/pickup ~0 (0 сэмплов).
- **H0-CENS**: два суб-таргета ≥3% ЕСТЬ, но оба соло sub-бар → честный потолок ниже.

## Топ-суб-таргеты (новое знание против «чёрного ящика ItemEntity 20%»)
1. **item-fluid-скан 7.3-8.1% total CPU**: каждый item каждый тик сканирует жидкостное
   окружение (updateInWater/eyes/getFlow), хотя item лежит на земле и fluid-состояние
   секции меняется редко. СУЩЕСТВУЮЩИЕ лейны НЕ таргетируют этот сайт: fluid_guard
   (CRUSSTY_FLUID_PUSH_GUARD) = push-гейт; fluid_bitmask #16 = FluidPushGuardHook data-plane;
   fluid_dirty #6 = FluidPushOps.scan мемо. Сайт Entity.updateInWaterStateAndDoFluidPushing
   под ItemEntity.tick — свободная вилка «item-fluid-dirty» (dirty-stamp по секции).
2. **checkInsideBlocks residual 4.35-4.51% ПРИ inside_cache=1**: гейт (InsideBlockOps.gate
   0.36-0.52% виден в профиле) кэширует discovery, но свип forEachBlockIntersectedBetween
   жив на каждом тике. Кандидат на остаток: inside_bitmask lever #15 (section all-air
   pre-gate), на этих ногах 0-armed («OPTION-B FLAGMAN» — ждёт санкции владельца).
3. **физика боксов 5.1-5.4%** (move/collide + noCollision block-shapes; AABB+Vec3 alloc) —
   класс zero_cursor/travel_diet, третье место.

## Capture-матем потолка (честная, capture≈CPU-share, k=1.0; σ_run TPS@dp50k 17% — AG-16)
- соло item-fluid-dirty: устранение до ~80% остатка ≈ 5.8-6.5% CPU ≈ ΔMSPT -15-19ms на
  avg 252-295ms → TPS +6-8% — **соло sub-бар** (+20% бар S#3 = 4.32 при σ-базе).
- комбо fluid+inside+move: суммарно ~13-16% CPU → **+13-18% TPS** — у бара, но соло-леверов
  не хватает; комбо-вилка на волну-527 (3 lever'а или битмап-пре-гейт).
- Полное устранение ItemEntity.tick невозможно (физика+despawn обязательны) —
  верхняя граница всей оси ≤ ~+18-20%, реалистично +8-15%.

## Числа-факты
- runs: 36971367106 (AG-22a) + 36971305525 (AG-37b), арты re-harvest 2026-10-02,
  artifacts id 11217330861 / 11217940138.
- Дельты ног по всем суб-путям ≤0.6пп — суб-профиль dp50k стабилен (в отличие от TPS σ17%).
