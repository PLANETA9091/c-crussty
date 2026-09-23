# RECON-35 — дрилл block-read семьи до доменов-потребителей + вердикт chunk-read-диеты

## s7194 (сцена 128104 сэмплов)

### домены-потребители block-read семьи

семья целиком: 9659 = 7.54% сцены

| домен | сэмплов | % семьи | % сцены |
|---|---|---|---|
| fluid-sim | 4824 | 49.9% | 3.77% |
| movement-collision | 2115 | 21.9% | 1.65% |
| entity-context | 1595 | 16.5% | 1.25% |
| inside-discovery | 988 | 10.2% | 0.77% |
| pathfinding | 130 | 1.3% | 0.10% |
| other | 7 | 0.1% | 0.01% |

### read-листья (топ-8)

- 46.22% семьи   4464 get
- 20.36% семьи   1967 getFluidState
- 13.12% семьи   1267 getBlockState
-  8.38% семьи    809 readPalette
-  7.01% семьи    677 getBlockStateFinal
-  2.26% семьи    218 getSections
-  0.81% семьи     78 getMinY
-  0.61% семьи     59 getBlockStateOnLegacy

### ближайшие вызывающие кадры (топ-10, полные имена)

- 16.84% семьи   1627 updateFluidHeightAndDoFluidPushing
- 15.28% семьи   1476 getFlow
- 13.57% семьи   1311 hasSameAbove
-  9.10% семьи    879 getCollisionsForBlocksOrWorldBorder
-  7.69% семьи    743 lambda$checkInsideBlocks$2
-  3.12% семьи    301 isValidTarget
-  2.73% семьи    264 getBlockSpeedFactor
-  2.68% семьи    259 getOnPos
-  2.66% семьи    257 getFloorLevel
-  2.51% семьи    242 updateFluidOnEyes

## s7189 (сцена 127150 сэмплов)

### домены-потребители block-read семьи

семья целиком: 9353 = 7.36% сцены

| домен | сэмплов | % семьи | % сцены |
|---|---|---|---|
| fluid-sim | 4584 | 49.0% | 3.61% |
| movement-collision | 2076 | 22.2% | 1.63% |
| entity-context | 1571 | 16.8% | 1.24% |
| inside-discovery | 976 | 10.4% | 0.77% |
| pathfinding | 143 | 1.5% | 0.11% |
| other | 3 | 0.0% | 0.00% |

### read-листья (топ-8)

- 47.15% семьи   4410 get
- 18.89% семьи   1767 getFluidState
- 13.54% семьи   1266 getBlockState
-  8.25% семьи    772 readPalette
-  7.15% семьи    669 getBlockStateFinal
-  1.99% семьи    186 getSections
-  0.75% семьи     70 getBlockStateOnLegacy
-  0.72% семьи     67 getMinY

### ближайшие вызывающие кадры (топ-10, полные имена)

- 16.89% семьи   1580 updateFluidHeightAndDoFluidPushing
- 14.50% семьи   1356 getFlow
- 13.23% семьи   1237 hasSameAbove
-  9.31% семьи    871 getCollisionsForBlocksOrWorldBorder
-  7.94% семьи    743 lambda$checkInsideBlocks$2
-  3.13% семьи    293 getOnPos
-  3.09% семьи    289 isValidTarget
-  2.47% семьи    231 move
-  2.44% семьи    228 updateFluidOnEyes
-  2.42% семьи    226 getBlockSpeedFactor

## ВЕРДИКТ (клейм RECON-29 ~3.6% ЗАНИЖЕН ~2×; самостоятельного рычага НЕТ)

1. Block-read семья (PalettedContainer/* + Level[/]Chunk[/]
   getBlockState*/getFluidState + ChunkAccess/* + readPalette*,
   deepest-wins, полный кадр) = **7.36..7.54% сцены** — клейм RECON-29
   «chunk-read ~3.6%» занижен ~2×: узкие регексы пропускали
   Level-делегаторы (getBlockState/getFluidState на Level),
   внутренние кадры PalettedContainer/ChunkAccess (get/getSections/
   getMinY/getNonEmptyBlockCount) и getBlockStateFinal.
2. Семья = ЧИСТАЯ ИНФРАСТРУКТУРА потребителей (кросс-стабильно
   s7194/s7189): **fluid-sim ~50% семьи = 3.61..3.77% сцены**
   (updateFluidHeightAndDoFluidPushing 16.8% + getFlow 15.3% +
   hasSameAbove 13.6% + updateFluidOnEyes 2.5% — REFUTED-зона #10,
   ванильная семантика растекания/плавания, parity-критично);
   **movement/collision ~22% = 1.63..1.65%** (getCollisionsForBlocksOr
   WorldBorder 9.1% + move/travel/getBlockSpeedFactor — лейн #14 REFUTED);
   **entity-context ~17% = 1.24..1.25%**; **inside-discovery ~10% =
   0.77%** (уже покрыт dormant-флагманом #15 CRUSSTY_INSIDE_BITMASK,
   option B); **pathfinding ~1.4% = 0.10..0.11%**. НИ ОДИН потребитель
   ≥5% сцены вне уже отображённых/закрытых лейнов.
3. Чтение как таковое (листья): PalettedContainer.get ~3.49..3.47% сцены
   (4410..4464), getFluidState ~1.54..1.39%, getBlockState ~0.99..1.00%,
   readPalette ~0.63..0.61%, getBlockStateFinal ~0.53% (final-state кэш
   уже в ваниле), getSections/getMinY <0.3% (section-lookup инфра).
   Мемоизация чтений = movement-invalidation-аналог (RECON-30): потолок =
   полная ликвидация семьи <7.6% сцены < ДВОЙНОГО БАРА +10%; по эмпирике
   6 REFUTED-ног TPS-конверсия диет-семейства ~0 → микро-зона чартера.
4. ВЕРДИКТ: chunk-read-диета НЕ БАНКИНГУЕТСЯ — 9-е ДОК-ЗАКРЫТИЕ
   (RECON-17/20/23/26/30/31/32/33/35); последний несведённый компонент
   ЭПОХИ-2A сведён: вариант A исчерпан полностью, рекомендация владельцу
   B/C из RECON-29×32 усилена. При option B флагман #15 уже покрывает
   внутри-discovery-чтения (0.77% сцены + ~9.1% CPU всей inside-семьи);
   fluid-чтения = ванильная семантика (не убираются при парити).
   Честный резидуал: атрибуция доменов по стекам (JIT-инлайнинг может
   смазать границы потребителей), но кросс-лег-стабильность
   (7.54/7.36%) + чистые списки вызывающих дают уверенность в раскладе.

## Пины
- данные: run-s7194-zeroalloc-v1 / run-s7189-traveldiet-v2a
  world3-bench.zip → cpu-collapsed.txt (spark, cpu-окно; авто-распаковка
  в /tmp/recon35)
- скрипт: scripts/bench4_recon/recon35_chunkread_consumer_drill.py
- прецеденты: RECON-29 (клейм), RECON-30/31/32 (методика дриллов)

