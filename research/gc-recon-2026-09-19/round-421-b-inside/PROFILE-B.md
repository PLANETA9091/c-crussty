# PROFILE-B — TASK-421-B ре-профиль inside-подсистемы на носителе 2d23f45

Источник: артефакты якорей-421 (vanilla lever='' @2d23f45).
- round-421-anchorb run 35800926548: TPS med 2.15 @ runner 6795554, samples 115755 (ABSORB: /home/z/c-crussty/research/gc-recon-2026-09-19/round-421-anchorb/)
- round-421-anchorc run 35800931828: абсорбирован позже (см. worklog; добавлен как вторая точка)
- round-421-anchora run 35800920567: завершается ~08:50 +08 (третья точка)

## Лейны на якоре (vanilla, anchorb, 115755 сэмплов)

| лейн | был (базлайн 115655) | стал (anchorb) |
|---|---|---|
| items | 31.17% | 28.86% |
| fluid | 16.72% | 15.61% |
| broadphase | 15.66% | 16.19% |
| nav_ai | 14.16% | 14.26% |
| inside_volatile | 12.01% | **10.68%** |
| fastutil | 8.54% | 8.39% |
| java_util | 7.01% | 6.29% |
| paletted | 6.41% | 6.05% |

ВАЖНО: якоря идут с lever='' — слитые плоскости НОСИТЕЛЯ (items_manager, queryplane и т.д.) спят, поэтому items 28.86% (не ~0 как на ARMED-ногах mg420d). На ARMED-носителе (mg420d, lever=cmp420_colpush) inside_volatile = 12.26%. Оба числа ≥8% → **ПИВОТ НЕ НУЖЕН, реализуем inside-плоскость**.

## Классификация лист→корень inside-лейна (10.68% = 12358 сэмплов, срез до последнего ServerLevel.tick)

Метод: collapsed-стеки, содержащие checkInsideBlocks|collidedWithShapeMovingFrom ниже последнего ServerLevel.tick.

Срезы (доля ОТ ОБЩЕГО CPU):
- BlockGetter.forEachBlockIntersectedBetween (оркестрация traversal): **8.045%**
  - visitor lambda$checkInsideBlocks$2: **5.072%** (getState/shape/fluid/effects — ванильное тело)
  - BlockPos.betweenCornersInDirection: 1.311% (+ lambda$8 0.158, iterator 0.158)
  - LongOpenHashSet.add 0.771% + init 0.257% + clear 0.120% ≈ **1.15%**
  - BlockPos$6.computeNext (betweenClosed): 0.689%
  - addCollisionsAlongTravel: 0.609% + getFurthestCorner 0.126%
  - Mth.floor 0.308, Vec3.add 0.206, AABB.<init> 0.208, MutableBlockPos.set 0.177, asLong 0.192
- ЧТЕНИЯ СОСТОЯНИЙ (volatile-срез PalettedContainer):
  - Level.getBlockState (mid): 1.027% → LevelChunk.getBlockState 0.593 / getBlockStateFinal 0.498
  - PalettedContainer.get: 0.487% + SimpleBitStorage.get 0.126% + readPalette 0.099% ≈ **0.71%**
- FLUID-подсрез лейна (НЕ мой скоуп — fluid-плоскости закрыты владельцем ×4): FluidState.getAABB 0.993 + getHeight 0.751 + Level.getFluidState 0.521 + LevelChunk.getFluidState 0.701 + getType 0.156 ≈ 3.1%
- Коллектор (armed-плоскости batch_collector/flush_diet, не мой скоуп): BatchCollector.advanceStep 0.867% + flushStep 0.765% + ArrayList.isEmpty 0.359%
- InsideBlockOps.gate (armed inside_cache): 0.859% self

## Вердикт профиля → дизайн плоскости

Единственный ЗВУКОВОЙ батч-таргет внутри лейна = **volatile-чтения состояний блоков** (PalettedContainer.data — буквально volatile-поле; бит-декод SimpleBitStorage.get + readPalette-дереф на КАЖДУЮ visit-позицию движущейся сущности). Оркестрация traversal (betweenCorners/DDA/LongOpenHashSet) — вводы движущихся сущностей меняются КАЖДЫЙ тик (from/to определяются в тике) ⇒ прe-tick батч позиции-списков НЕЗВУКОВОЙ (ghost ≠ снапшот) — это территория запрещённого flat_traversal #9 и причина его парковки. Fluid-подсрез и коллектор — закрытые/чужие armed-плоскости.

**Дизайн cmp421_inside (батч-снапшот volatile-состояний)**: пер-секционные снапшоты BlockState[4096], собираемые ОДНИМ bulk-JNI за тик (буфер входов = flat words/bpe новых секций → rust palette_gather bit-exact декод → готовые выходы = flat индексы → java-скэттер в BlockState[4096]), инкрементальные (steady-state ≈ 0 сборов), инвалидация event-driven (retarget единственного сайта LevelChunkSection.setBlockState в LevelChunk.setBlockState → bump генерации, FluidPushOps.secWrite-паттерн), гейт на ЕДИНСТВЕННОМ сайте Level.getBlockState в Entity.lambda$checkInsideBlocks$2 (receiver-first 3B→3B). Сервинг HIT = `palette[(y&15)<<8|(z&15)<<4|(x&15)]` — ТОТ ЖЕ объект, что возвращает ванильный readPalette (moonrise$getPalette()). Ожидаемый срез лейна ≈ 1.5-2пп (12.26 → ~10.3-10.8 на ARMED-носителе), дельта плоскости ≈ +1-2% TPS поверх носителя.

Адресуемая база среза: PalettedContainer.get 0.49 + SimpleBitStorage.get 0.13 + readPalette 0.10 + LevelChunk.getBlockStateFinal 0.50 + часть LevelChunk.getBlockState 0.59 + Level.getBlockState self ≈ **1.3-1.6% total CPU**.
