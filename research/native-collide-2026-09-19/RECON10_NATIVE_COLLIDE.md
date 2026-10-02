# RECON-10 — рычаг #12 NATIVE-COLLIDE: javap-контракт лестницы collide + профильная раскладка → GO/NO-GO

Тик 12:43 +08 2026-09-19 (TASK-315). База: банковый профиль v3 (run 35399980345, cpu-collapsed 128124 сэмплов, урок №6) + javap-дизассемблирование реальных kernel-классов (tests/fixtures — байт-в-байт kernel classfiles: CollisionUtil = `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil`, 45439 байт; Entity_real = патченный Entity, 205458 байт).

## 1. Методика

- javap -p -c (Temurin 21, /tmp/jdk21) по фикстурам; per-method парсер инструкций (scripts/recon10_collision_bytecode.py → cu_full.txt, entity_full.txt, recon10_methods.json).
- «Юнит» = 1 инструкция байткода (честная прокси длины Code-атрибута).
- INCLUSIVE-ценз поддеревьев по cpu-collapsed v3 (recon10_inclusive.py) — решающая ось: где реально сидит CPU.
- Self-ценз листьев (recon10_cpu_split.py).

## 2. javap-контракт лестницы (per-method, юниты)

| метод | юниты | dcmp | eps 1.0E-7 | внешние вызовы |
|---|---|---|---|---|
| performCollisions(Vec3,AABB,List<VS>,List<AABB>) | 120 | 8 | 0 | Math ×2, Vec3.<init> ×1 |
| performVoxelCollisions / performAABBCollisions | 92+92 | 8+8 | 0 | Math, Vec3.<init> |
| performVoxelCollisionsX/Y/Z (лупы) | 22×3 | 0 | 0 | collideX/Y/Z(VS) |
| performAABBCollisionsX/Y/Z (лупы) | 22×3 | 0 | 0 | collideX/Y/Z(AABB) |
| **collideX/Y/Z(VoxelShape,AABB,d)** | **294×3=882** | 5×3 | 8×3 | Math ×8, findFloor ×6, **CachedShapeData ×4** (плоские данные: коорд-массивы) |
| collideX/Y/Z(AABB,AABB,d) | 70×3=210 | 7×3 | 6×3 | Math (чистая double-математика) |
| findFloor(double[],d,d,i,i) | 40 | 1 | 0 | — |
| isEmpty ×2 | 50 | 6 | 6 | — |
| voxelShapeIntersectNoEmpty | 197 | 0 | 6 | findFloor ×6, CachedShapeData ×4 |
| **ИТОГО лестница-ядро** | **≈1568** | | | **0 world/border-вызовов** (только плоские данные CachedShapeData) |

World/border-часть:

| метод | юниты | внешние вызовы |
|---|---|---|
| **getCollisionsForBlocksOrWorldBorder** | **519** | Level ×2, WorldBorder ×2, WorldUtil ×2, BlockState ×3, ChunkSource ×1, LazyEntityCollisionContext, Mth ×6 — **>400 порога** |
| getEntityHardCollisions | 78 | Entity ×4, Level ×1 |
| isCollidingWithBorder ×2 | 55 | WorldBorder ×4, Math ×4 |
| Entity.collide (орchestration) | 265 | CollisionUtil ×7, ArrayList ×4, calculateStepHeights |
| Entity.move (орchestration) | 560 | Vec3 ×11, Level ×4, Mth ×5 (в основном не collide-математика) |

Пороговые константы лестницы: COLLISION_EPSILON = 1.0E-7 (14 вхождений в collideX/Y/Z-VS, 6 в AABB-вариантах, 6 в getCollisionsForBlocksOrWorldBorder); dcmpg-лестницы в collide* (5-7 на метод) — повторение урока №9 (clipPoint): зеркалить семантику dcmpg/dcmpl бит-в-бит.

## 3. INCLUSIVE-раскладка CPU (v3-банк, 128124)

| поддерево | сэмплы | % CPU |
|---|---|---|
| **лестница performCollisions (вся X/Y/Z + collide*)** | **474** | **0.37%** (voxel-ветка 0.00%, AABB-ветка 0.23%) |
| **сбор getCollisionsForBlocksOrWorldBorder** | **4416** | **3.45%** |
| getEntityHardCollisions | 2158 | 1.68% |
| isCollidingWithBorder | 51 | 0.04% |
| PalettedContainer.get (все потребители) | 5546 | 4.33% (self 2.96% — residual после банked PALETTED-DEMUX S7-131) |
| Entity.collide / Entity.move / collideBoundingBox | 0 | 0.00% — целиком инлайнены C2 в вызывающие фреймы |

Self-CPU лестницы 0.27% (findFloor 0.118 — максимум) — C2 инлайнит лестницу в оркестрацию, её стоимость уже растворена в caller-фреймах.

## 4. GO/NO-GO — ВЕРДИКТ: ЗАКРЫТО (двойное якорение)

**#12 NATIVE-COLLIDE = INFEASIBLE-BY-VERBATIM (по сбору) + PAPER-REFUTED (по лестнице). БЕЗ CI-бута, бюджет сохранён.**

1. **Ось потребления vs ось портабельности расходятся**: чистая double-математика (1568 юнитов, 0 world-вызовов) — портабельна, но держит ≤0.37% CPU (в 5-10 раз ниже запрещённого владельцем класса 2-3% MSPT). Реальные потребители — сбор блоков: getCollisionsForBlocksOrWorldBorder (3.45% + hard-entities 1.68% ≈ 5.1%) — 519-юнитный метод С внешними Level/ChunkSource/BlockState/WorldBorder-вызовами → **порог 400 превышен → вербатим-порт INFEASIBLE** по прereg-критерию TASK-314.
2. **«Буфер палитр/секций» (спека TASK-314) не спасает**: перенос сбора в Rust = реимплементация (не вербатим) диспетчера BlockState→shape (moonrise BlockStateShapeCache), LazyEntityCollisionContext, WorldUtil-границ, ChunkSource-ретрива — вне класса эры «bit-exact вербатим» по parity-риску.
3. **Даже лестничный Option A рефутуется экономикой до сборки**: JNI-граница теряет C2-инлайн лестницы в Entity.collide/move (сегодня self-CPU 0.27% = стоимость уже растворена инлайном) — повторение урока №8 на границе JNI + сериализация shape-данных (CachedShapeData коорд-массивы) на сущность/тик. Ожидаемый знак эффекта — отрицательный, как в #10.

Экономия CI-лега: третий подряд paper-вердикт эры (#11 travel, #12 native-collide; прецедент RECON-9).

## 5. Свежий ТОП по self-CPU (v3-банк; перестройка «→ ∞» после закрытия #12)

ТОП листьев self-CPU (GC-фаза отдельно): G1-внутрянка ≈25-30% суммарно (OopOopIterateDispatch 4.28+2.42+1.35+1.07+0.97, oopDesc::size 4.01, G1CardSet 3.34+1.52+1.25, trim_queue 1.69, G1RemSet 1.54, G1ScanCard 1.26, scan_heap_roots 0.94, RebuildRS 0.70) — даунстрим аллокации.

Небовый (не-GC) self-ТОП:
1. **блочные чтения ≈5.5%** (PalettedContainer.get 2.96 + readPalette 0.68 + SimpleBitStorage.get 0.64 + getBlockStateFinal 0.65 + getFluidState 0.57) — ПОСЛЕ банked PALETTED-DEMUX (исходный ТОП-1 владельца, −15% лейна); residual = пост-оптимизационный пол трёх потребительских лейнов (collision-сбор 3.45%, inside, fluid)
2. updateFluidHeightAndDoFluidPushing 1.97% (тело пережило REFUTED #10 — механика не банковалась)
3. entity-broadphase ≈2.1% (ConcurrentLong2ReferenceChainedHashTable.getNode 1.20 + ChunkEntitySlices.getEntities 1.16+0.94) — 2×REFUTED, ПАРК
4. AABB-геометрия ≈2.2% (intersects 1.13 + <init> 1.06) — temps-поверхность исчерпана (RECON-9)
5. SynchedEntityData ≈1.5% (VarHandle 1.04 + getItem 0.73 + getValue 0.78) — гетерогенный entity-other (RECON-8, без ≥5% атакующих)
6. setDeltaMovement 0.93%, Mth.floor 0.79%, Reference2ObjectOpenHashMap.get 0.91%

Классы миссии закрыты все: кэш/мемоизация (inside_cache/demux/area-map банked; fluid_dirty hit-rate≈0), батч-операции (batch_collector 2×REFUTED; flush_diet банked), event-driven dirty-flags (fluid_dirty REFUTED), O(n)→O(1) (demux, inside_cache), Rust/JNI (#12 INFEASIBLE — этот тик), планировщики (region_threads=4 банked).

## 6. NEXT (тип+1, RECON-11)

ТОП-1 по self-CPU = G1/GC-фаза ≈25-30% — даунстрим аллокационного давления; прямые аллок-поверхности исчерпаны (RECON-9: потолок ≤0.83% wall на не-атакованной поверхности 11.7%). RECON-11: (а) разложить G1-фазу на драйверы (какие аллок-семьи кормят oop-iterate/card-set: типы объектов, средний размер, выживаемость young); (б) трёхосевая пересортировка от свежих артефактов (урок №7: фильтрация сер-стеков концевого сейва); (в) кандидаты #13: (1) LESSON-8-COMPLIANT редирект ТОЛЬКО крупных не-инлайнибельных аллок-тяжёлых тел (прецедент: updateFluid уже скаляризован как нейтральный — пересмотреть гейт под аллок-цель, не CPU-цель); (2) демux-v2 residual-лейна (getFluidState/getBlockStateFinal вне demux-пути); (3) G1-френдли предвыделение в горячих циклах (объект-пул типа collision-листов) — класс «планировщик/кэш», НЕ редирект. Порог отбора прежний: ≥5% лейн или ≥2-3%-класс wall-эффект, vanilla-parity, иначе paper-REFUTED без CI-бута.
