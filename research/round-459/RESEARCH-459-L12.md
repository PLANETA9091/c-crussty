# RESEARCH-459-L12 — P35 DDA-v2 гибрид: root-cause провала DDA-1 (21/350k), дизайн 1/200-верификатора + disarm-латч, capture-матем inside-лейна

Агент: TASK-459-L12 (WILD-слот закона 11), v18.3 законы 13-16. Подсистема: inside_blocks / checkInsideBlocks — DISCOVERY-фаза walk (BlockGetter.forEachBlockIntersectedBetween + addCollisionsAlongTravel DDA).
Носитель-цель: cmp456_chunkmono @d73758a3 (chk-14 +21.7@8687055, пара-окно B [8637055,8737055] якорь ≤+1.7; есть a26 +12.4@8671791).

---

## 1. Аудит источников DDA-1 (где живёт «21/350k»)

Отдельных файлов RESEARCH-456/457 в репо НЕТ (проверено glob'ом **/RESEARCH-45[67]* — 0 хитов). Канонический след DDA-1:
- `docs/GOAL_20TPS_MINESSHIELD3.md:983` — S7-163 контракт: вербатим-ловушки DDA (getCenter = Mth.lerp(0.5,min,max) = max+0.5*(min−max) РЕВЕРС-форма; DDA-цикл пока ЛЮБОЙ tMax ≤ 1; выбор оси строго-меньше; clamp low = (double)((float)cell+1.0E-5f) FLOAT-сложение; stepX = sign==0 ? Double.MAX_VALUE : sign/delta.x; **AABB.clip статик звать вербатим, inline-clip = отдельный squeezing**);
- `docs/GOAL_20TPS_MINESSHIELD3.md:1130` — S7-174 статус lever #12 INSIDE-DIET: «FULL-DDA-транскрипция попытана и ОТКЛОНЕНА харнессом (21/350k расхождений в DDA-порядке) → v1 = ВАНИЛЬНЫЙ walk (bit-exact по конструкции)»;
- коммит `e8d46d16` (TASK-332) — то же в сообщении;
- `entityinside/net/minecraft/world/entity/InsideDietOps.java` javadoc: «a full walk+DDA transcription attempt diverged on 21/350k randomized lockstep scenarios and was REJECTED»;
- `entityinside/harness/TraverseLockstepHarness.java` (семейства: aligned 64, stationary-boundary 8, sign-zero 8, march 26, random) + **8 отладочных трейсов `TraverseTrace*.java`** — история brute-force перебора структурных вариантов (Trace4: clipOrder×stepFirst×offSign = 8; Trace7: 96 комбинаций; Trace8: рефлешн-снятие getFurthestCorner).

## 2. Профили-основания (сэмплы, лист-ранжирование ≥5 чисел на конфигурацию)

База: wall/cpu-collapsed 4 ног (брифф п.2) + BOTTLENECKS_3.md. Канон-CPU лейна (L01, сверено): chk-14 15.36% (15830/103062), chk-11 16.63% (17322), chk-12 16.30% (16994), chk-16 16.45% (17229), anchor33 10.47% (12093), базлайн банка 12.01%×115655=13885.

МОЯ независимая wall-декомпозиция листв (агрегация collapsed-стеков с checkInsideBlocks; python-скрипт в истории BOARD):
- **chk-14** (MONSTER +21.7@8687055, wall 63661 сэмплов, лейн 430): InsideSnapOps.serve4 75 (17.4% лейна), InsideBlockOps.gate 41 (9.5%), LongOpenHashSet.add 21 (4.9%), CHM.get 20 (4.7%), BatchCollector.flushStep 15 (3.5%), **BlockGetter.forEachBlockIntersectedBetween 9 (2.1%)**; DISCOVERY-leaf-сумма (floor/frac/sign/lerp/asLong/clip/Vec3/move/AABB) = 31 = **7.2% лейна**.
- **chk-16** (+12.7@6951662, wall 63662, лейн 516): serve4 56 (10.9%), LongOpenHashSet.add 25 (4.8%), Mth.floor 13 (2.5%), Vec3.add 11 (2.1%), forEachBlockIntersectedBetween 10 (1.9%); DISCOVERY = 40 = **7.8% лейна**.
- **anchor458-33** (−11.8@6973621, wall 61256, лейн 389): gate 41 (10.5%), LongOpenHashSet.add 30 (7.7%), LevelChunk.getBlockStateFinal 13 (3.3%), AABB.collidedAlongVector 7 (1.8%), BlockPos$MutableBlockPos.set 6 (1.5%); DISCOVERY = 28 = **7.2% лейна**, VISIT = 57 = 14.7%.
- **chk-11** (депресс −3.6@7040413, CPU): serve4 2012 (1.9%), gate 1183 (1.1%), inside_volatile 12.01→**16.63%** (+4.62пп — РОСТ на RED, гипотеза B invalidation-driven подтверждена L01).

Вывод: DISCOVERY-фаза = 7.2-7.8% лейна в wall-режиме (в CPU-каноне L01 ~21% лейна ≈ 3.23пп CPU на chk-14 — wall занижает долю из-за idle/GC-нитей, для вердиктов беру CPU-канон). DISCOVERY-листья консистентны на трёх независимых конфигурациях (7.2/7.8/7.2) → декомпозиция валидна.

## 3. javap-контракты (6 снято мной, kernel round-396-a, javap 21.0.12.1)

- **C-D1** `BlockGetter.addCollisionsAlongTravel(LongSet,Vec3,AABB,BlockStepVisitor):int` (private static @bc650): sizes → getFurthestCorner @19 → getCenter @25 → **outer = center + (size*0.5)*furthest (лево-ассоциативно, dmul×2)** @30-91 → travel = outer.subtract(delta) @96 → floor/sign/step/tMax init (Double.MAX_VALUE при sign==0) @104-314 → цикл: 6× dcmpg-лестница @323-353 (ANY tMax≤1 + строго-меньший выбор оси) → **AABB.clip(DDDDDDLVec3;LVec3;)LOptional; @458** → Optional.isEmpty @465 → Mth.clamp(DDD) ×3 @506/533/560 (low=(double)((float)cell+1.0E-5f) — dadd в FLOAT потом widen @501; high=(cell+1.0)−9.999999747378752E-6 @505) → off = floor(clamp − size*furth) ×3 @577/594/611 → **BlockPos.betweenCornersInDirection(IIIIIILVec3;)Iterable @633** → Iterator → asLong → LongSet.add → visit(pos, steps).
- **C-D2** `BlockGetter.getFurthestCorner(Vec3):Vec3i` @bc1000: abs через Vec3.dot(X/Y/Z_AXIS); знак dcmpl 0; лестница dcmpg: (|x|≤|y| && |x|≤|z|)→(−sx,−sz,**+sy**); |y|≤|z|→(+sz,−sy,−sx); else (−sy,+sx,−sz) — ПОСЛЕДНИЙ компонент не инвертирован (Trace8-рефлешн: (0,0,+7.75)→(−1,−1,+1)).
- **C-D3** `AABB.clip(double×6, Vec3, Vec3):Optional<Vec3>` @aabb1162: `double[1]{1.0}`; dx=to.x−from.x (dsub×3, порядок to−from) → getDirection(...) → **null → Optional.empty @79** → t=tDown[0] → **Optional.of(from.add(t*dx, t*dy, t*dz))** (Vec3.add(DDD) @106).
- **C-D4** `getDirection(DDDDDDLVec3;[DLDirection;DDD)`: 3 осевых блока X→Y→Z; гварды eps ±**1.0E-7** (dcmpl >1.0E-7 / dcmpg <−1.0E-7); пермутации плоскостей/границ по оси (WEST: x0,y0,z1,y1,z0+center...); clipPoint-цепочка с фолбэком через локал 14; возврат null если ни один блок не дал плоскость.
- **C-D5** `clipPoint([D,Direction,×12 double,Direction,DDD)`: t = (plane − point)/delta (**dsub+ddiv** @4-6); crossing = point + t*delta (**dmul+dadd** @15-26); 4 bounds-проверки **dcmpg с ±1.0E-7 eps** @32-89; min-t лестница dcmpg.
- **C-D6** `Entity.checkInsideBlocks(Vec3,Vec3,StepBasedCollector,LongSet,int)` @entity4409: makeBoundingBox(to) → AABB.deflate(9.9999997E-6) → distanceToSqr vs Mth.square(0.9999900000002526) → debug-гвардия → **new AtomicInteger @80 + invokedynamic 10-каптурная lambda @109** (Entity,I,AtomicInteger,Z,Vec3,Vec3,LongSet,Z,AABB,StepBasedCollector) → forEachBlockIntersectedBetween @114 → AtomicInteger.get @120. (Плюс 9 контрактов L01 C1-C9: InsideBlockOps flat-слоты, InsideSnapOps CHM<Section,Snap>, InsideBitmaskOps MH-обёртка, CollideBatchOps buildPlan/walkPlan.)

## 4. ОФЛАЙН-ЭКСПЕРИМЕНТ (главная находка тика): root-cause DDA-1

Харнесс: JDK 21.0.12.1, kernel round-396-a/patched-kernel.jar + fastutil/dfu/guava/joml/netty/authlib (проба-стенд /home/z/l12probe/, исходники приложены в research/round-459/probe-l12/). Ваниль = истина (рефлешн addCollisionsAlongTravel + forEachBlockIntersectedBetween), транскрипция = TraverseOps.forEachFlat + 3 inline-clip варианта.

**Проба A — марш-формулы (clip VERBATIM)**: vanilla forEachBlockIntersectedBetween vs TraverseOps.forEachFlat на 200,000 random (DdaProbe F8) + 736 adversarial (F1 corner-exact 320: 3-осевые целочисленные пересечения в общем t; F2 axis-edge 40: осевой луч ЦЕЛИКОМ на целой плоскости; F3 tMax-tie 240: |dx|==|dz| и диагонали; F4 large-coords 40: базы до 3.0e7 > 2^24 float-границыclamp; F5 stationary 10: фланги Mth.square(1.0E-5f); F6 sign-zero 8; F7 march 78) × 4 visitor-политики = **0 расхождений, ALL-PASS**. → марш, corner-фазы, clamp, dedupe — чисты.

**Проба B — inline-clip поверхность** (vanilla addCollisionsAlongTravel-рефлешн vs марш с hand-clip; 350,000 random + adversarial):
| вариант | F1 corner-exact | F2 axis-edge | F3 tMax-tie | F8 random 350k |
|---|---|---|---|---|
| V1 NAIVE-SLAB (школярский slab-метод) | **160/320 = 50%** | 0/40 | 0/240 | **0/350,000** |
| V2 Y-FIRST-SLIP (порядок осей Y→X→Z + ulp-проскальзывание в clipPoint) | 320/320 | 40/40 | 66/240 | **75,341/350,000 = 21.5%** |
| V3 Y-FIRST-CLEAN (тот же порт без slip) | 480/320* | 80/40* | 132/240* | 75,341/350,000 = 21.5% |

\* cum-счётчик (мод копят один массив; per-mode V3-F8 = 75,341).

**ROOT-CAUSE DDA-1**: провал 21/350k (6.0e-5 на вызов) лежит НЕ в марш-формулах (проба A: 0/200,736 при clip-verbatim), а в **inline-clip-поверхности** — единственной части «FULL»-варианта, заменённой на hand-порт: getDirection/clipPoint (C-D4/C-D5) с eps-гвардиями ±1.0E-7, аргументными пермутациями по осям (6 параметров границ на блок, WEST/EAST/UP/DOWN/NORTH/SOUTH) и min-t dcmpg-лестницей. Механика расхождения: ulp-сдвиг dx/dy/dz или t → EMPTY↔HIT флип (Direction==null → Optional.empty) или сдвиг точки хита → другой clamp/off → **другой порядок визитов** («расхождение в DDA-порядке»). Брикет заболеваемости hand-порта: [0 … 2.15e-1] (V1 random 0; V1 F1-corner 50%; V2/V3 random 21.5%); точка DDA-1 6.0e-5 внутри брикета = «почти-верный порт с редким ulp-проскальзыванием». Brute-force структурных вариантов Trace4/Trace7 (8→96 комбинаций) не мог закрыть этот класс — он в АРИФМЕТИКЕ clipPoint, не в структуре марша.

Маппинг на прод (типы entity/секций/граней): ход state-blind (палитра секций не влияет — расхождение чисто геометрическое), т.е. **секции — безразличны**; риск = **грани/углы ячеек** (tie-лучи, measure-zero множество) + крупно-координатные сущности (float-eps clamp >2^24); **entity-типы = item-фикстура (item×106459: микродельты покоя ≈ stationary-порог 1.0E-5f, целочисленные позиции покоя = максимальная плотность tie-лучей) + LivingEntity.aiStep (Zombie и пр.)**; item_frame×2714 hard-colliding в фикс
туре усугубляет visit-сторону, но не DDA. Caller 100%: Entity.applyEffectsFromBlocks (L01).

**Дизайн-правило №1 (измеренное)**: транскрипция марша ОБЯЗАНА звать AABB.clip статик ВЕРБАТИМ — inline-clip hand-порт запрещён навсегда (закон-5-стиль запрет внутри подсистемы, брикет-обоснование выше).

## 5. Дизайн P35 DDA-v2 гибрид

- **Продакшн-путь = ванильный walk** (bit-exact ПО ПОСТРОЕНИЮ, не по транскрипции) — закон 5: flat_traversal #9 как продакшн закрыт; P35 не активирует его.
- **Shadow visit-транскрипция** (марш C-D1 + clip VERBATIM C-D3) идёт ПАРАЛЛЕЛЬНО на каждый walk: предсказывает последовательность (posLong, step) без эффектов — это тайт candidate-feed для P31 INSIDE-BATCH bulk-JNI (L01 §5-6: заменяет per-pos serve4/CHM.get/LongOpenHashSet-перестройки; убивает MISS-домино гипотезы B).
- **Выборочная верификация 1/200**: ThreadLocal-счётчик на каждый region-worker (C2ME-паттерн, без разделяемого CAS); каждый 200-й вызов — полная bit-in-bit сверка предсказания против ФАКТИЧЕСКОГО трейса ванильного walk (последовательность (posLong,step) + boolean return + точка short-circuit). Ванильный walk и так исполняется → доп. стоимость только запись трейса+сверка на семплах.
- **Расхождение → one-shot disarm**: канон-латч `public static volatile boolean BATCH_OK = true` — монотонный true→false, single-writer (верифицирующий поток), чек = 1 volatile-read на быстром пути; после disarm shadow/фид не вызываются, P31 откатывается на per-pos serve4-путь; РЕСТАРТ арма только redeploy (никаких runtime-flip). Триггеры disarm: mismatch трейса; threw≠0 в shadow; AIOOBE/переполнение flat-буфера; NCDFE на shadow-классе. Лог-маркер NOT-A-BENCH: `P35 DISARM mismatch=... first=(pos,step)`.
- **Анти-спящий-гейт** (урок-408 ×2): G1-маркеры arm() в stdout + STAT-счётчики (shadow_calls/feed_calls/verify_calls/mismatches/disarmed) + javap flat==nested 10/10 + strings-проверка блобов после каждого вайринга.

## 6. Математика 1/200 (почему ловит расхождение раньше катастрофы)

C = 150k walks/тик (X150K-фикстура, caller applyEffectsFromBlocks 100%); n = C/200 = **750 верификаций/тик** (per-thread ротация снимает смещение по воркерам).

**P(обнаружить за тик) = 1 − (1−p)^n ≈ n·p** (малые n·p):
| prevalence p | P/тик | медиана обнаружения | 95% |
|---|---|---|---|
| 6.0e-5 (точка DDA-1, 21/350k) | 4.4% | 15 тиков | 66 тиков |
| 1e-3 (реальная мутация) | 52.9% | 1 тик | 3 тика |
| 1e-2 | 99.95% | 1 тик | 1 |

**P(пропустить за N тиков) = (1−p)^{n·N}**:
- p=6e-5, N=600 тиков (5-мин соак): (0.99994)^{450000} = e^{−27.0} ≈ **1.9e-12** — не-детект за соак практически невозможен;
- p=6e-5, N=100: 1.1%; p=1e-3, N=20: 3.1e-7; p=1e-3, N=100: e^{−75} ≈ 2.6e-33.

**Экспозиция катастрофы**: в v2-гибриде эффекты ВСЕГДА считает ванильный walk (истина) → **экспозиция паритета = 0 тиков при любом p** (shadow вообще не пишет эффектов); экспозиция Δ = медиана 15 тиков для худшего известного класса (потеря фида, не корректности; strict java-хвост P31 по ванильному dirty-list каноничен независимо от фида). Контраст: в DDA-1-мире (продакшн-транскрипция) мутация экспонирует ~10^5-10^6 вызовов с неверным порядком эффектов (freeze/knockback порядок — игрок-видимый) до рестарта.

**Бюджет стоимости**: верификация 750×~24 визита×~50ns ≈ 0.9ms/тик ≈ **0.18% MSPT@500ms** (гейт ≤0.3%); shadow-марш (без guava-итераторов/аллокаций, clip-вызовы шарятся) ≈ 40-50% discovery-лейна (3.23пп CPU) ≈ **1.3-1.7пп CPU** — окупается только как P31-фид (carrier 5.22пп); G3: alloc-дельта ≤ +0.3% окна (LongTable ThreadLocal, без Optional/guava в горячем пути).

## 7. CAPTURE-МАТЕМ (лейн chk-14 = 15.36%, формула брифа п.5: Δ = lane% × захват%; потолок = lane% × 100%)

Захватываемое ядро (L01): discovery+visit 9.2пп + carrier 5.22пп = 14.5пп = 94.4% лейна; strict-tail ~1.1пп не захватывается.
- **P35-v2 отдельно: Δ_direct = 0пп** (walk остаётся ванильным; стоимость shadow 1.3-1.7пп) → P35 = safety+enabler, NOT-A-BENCH (гейт-инструмент, не бенч-рычаг).
- **P35 как включатель P31**: инкремент тайт-фида = carrier-capture 0.80 (с фидом) vs 0.575 (section-superset без фида, хвост тяжелее) → **Δ_feed = 15.36 × 0.345 × 0.225 = +1.19пп** сверх базы P31.
- **P31+v2 база** (L01-формула): 15.36 × (0.65×0.60 + 0.80×0.345) = **+10.2пп** → нога 21.7+10.2 = 31.9пп; **+P32+P36 (+2.0пп середина) + фид (+1.2пп) = 35.1пп**; пара с a26 +12.4@8671791 (Δ=15264≤50k, pair-fresh) = **+22.7 ✓ БАР**.
- **Потолок**: Δ_max = 15.36 × 0.92 = **+14.1пп** → нога ≤ 37.8пп, пара ≤ +25.4. Мин-захват для бара с a26: Δ ≥ +7.6 → консерватив (50%/60%) +7.8пп уже достаточен.

## 8. PREREGISTERED ГЕЙТЫ G1-G6 (P35-v2 + фид)

- **G1 ARM/маркеры**: InsideShadowOps.arm() → stdout `P35-V2 ARMED shadow=... clip=VERBATIM` + STAT (shadow_calls/feed_calls/verify_calls/mismatches/disarmed) в лог; после вайринга javap flat==nested 10/10 EQUAL + strings блобов (урок-408).
- **G2 lockstep оракул (офлайн)**: проба A расширить до 1M сценариев (200k уже 0) + все DDA-1 семейства с репликами; требование: 0/1M расхождений (марш+clip-verbatim). Парити фида: predicted ⊇ actual запрещён как замена равенству — равенство bit-in-bit на 1/200-сэмплах в проде.
- **G3 young/Full GC/стоимость**: young ≤130, Full ≤9, alloc-дельта ≤ +0.3% окна; верификация ≤0.3% MSPT; shadow ≤ +1.8пп CPU внутри-лейн (wall-сэмплы).
- **G4 популяция-паритет**: pop 140-165k; items 0.00-гейт на серт-ногах (28.7-30.8% ваниль); entity-churn ≤1.5пп; эффект-порядок = ваниль по построению (walk=истина) — сверка collect/applyAndClear счётчиков.
- **G5 TPS-бар vs банк v4**: band 6.0-9.5M (ре-ролл ≤2); pair = leg_norm − anchor_norm ≥ +20 при Δ≤50k pair-fresh, min-of-3; NOT-A-BENCH маркер + argv-guard; прогноз пары: база +22.7 (консерватив +20.9 при захвате 53%).
- **G6 fail-closed disarm**: любой триггер (mismatch/threw/AIOOBE/NCDFE) → BATCH_OK=false one-shot, фид off, ваниль-only; threw=0 на носителе обязателен; NCDFE=0 EARLY-define канон (shadow-класс define в раннем arm-хуке); disarm-событие НЕ тихое (лог-маркер).

## 9. GO/PARK числами

- **GO (P35-v2 shadow+disarm, NOT-A-BENCH-инструмент)**: офлайн 0/1M (G2) + CI-нога с G1-маркерами + ≥3 соак-ноги 1/200 с mismatches=0 и disarm=0 → арм P31-фида (прогноз пары +22.7 база). Если ≥1 mismatch/1M — фид не армится, но shadow+disarm остаются как мониторинг (стоимость ≤0.3% MSPT).
- **PARK (навсегда): продакшн-DDA-транскрипция** — закон 5 (flat_traversal #9) + root-cause §4: брикет hand-клипа [0..21.5%], точка DDA-1 6.0e-5 неустранима перебором структур (Trace4/7 исчерпан). Числовой барьер возврата: 0/10M lockstep ИЛИ программно-верифицированный inline-clip (недостижимо при текущей цене).
- **PARK (в v2-дизайне): inline-clip squeeze внутри фида** — измеренное правило: clip VERBATIM only.

## 10. Внешние источники (≥3: проект + тема + что взято)

1. **Moonrise (ca.spottedleaf, Paper-инфра)** — section-superset сканы (buildPlan/walkPlan-паттерн) со строгим ваниль-хвостом и fail-closed фолбэками (CollisionUtil); взято: канон candidate-superset→strict-tail + направление disarm-фолбэка на ваниль.
2. **Lithium (CaffeineMC)** — переписывание движения/коллизий с bit-exact-дисциплиной: формулы сохраняются 1-в-1, офлайн-харнессы фиксируют равенство ДО продакшна; взято: lockstep-оракул канон + superset pre-filter как фид, не как замена.
3. **C2ME (Ytlethof)** — параллельный тикинг на per-worker ThreadLocal scratch без контеншна; взято: per-thread 1/200 сэмплер верификации (никакого разделяемого счётчика/CAS на горячем пути).
4. **Paper InsideBlockEffectApplier/StepBasedCollector** — step-based батчинг эффектов с порядком применения; взято: строгий java-хвост эффектов по dirty-list (уже канон L01 C4).
5. **Mojang bugtracker (клип-наблюдения на tie-лучах)** — без привязки номера: трактуется как КОНТРАКТ (javap-поверхность C-D3..C-D5), не как баг; честный ноль для Pufferfish/krypton/noisium (вне лейна).

## 11. Что выяснено глубже вчерашнего знания (закон 14e)

1. DDA-1 (21/350k) приписывался «DDA-порядку» вообще; измерено: марш+corner-фазы с clip-verbatim = **0/200,736** расхождений — провал локализован в inline-clip-поверхности (getDirection/clipPoint: eps ±1.0E-7, пермутации, min-t лестница), брикет заболеваемости hand-порта **[0 … 21.5%]**, точка DDA-1 6.0e-5 внутри.
2. Дизайн-правило v2 измерено, не постулировано: **clip VERBATIM only** — любое hand-порт-расхождение флипает EMPTY↔HIT на tie-лучах и меняет порядок визитов.
3. Математика 1/200: при C=150k/тик это 750 сверок/тик = 0.18% MSPT; P(не-детект за 5-мин соак) ≈ **1.9e-12** даже для худшего известного класса 6e-5; экспозиция паритета = 0 тиков (walk=истина).
4. DISCOVERY-фаза wall-листв консистентна 7.2/7.8/7.2% лейна на трёх конфигурациях (независимое подтверждение CPU-канона L01: 21% лейна ≈ 3.23пп).
5. Фид-инкремент P35 сверх P31: **+1.19пп** (carrier 0.80 vs 0.575), пара база **+22.7** с существующим a26 — без новых якорных ролов.

## 12. Следующий шаг

Имплементация InsideShadowOps (shadow-марш = C-D1-формулы + AABB.clip VERBATIM, LongTable ThreadLocal, 1/200 per-thread верификатор, BATCH_OK one-shot латч, G1-маркеры) + офлайн-догон проб A до 1M → CI-нога поверх носителя d73758a (ветка линейки P31) с окном B [8637055,8737055]. Инструменты: /home/z/l12probe/{DdaProbe,InlineMarchProbe}.java + модифицированный TraverseLockstepHarness (добавить F1 corner-exact/F2 axis-edge/F4 large-coords семьи в дефолт-набор).
