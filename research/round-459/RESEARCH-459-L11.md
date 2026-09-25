# RESEARCH-459-L11 — WILD H05: papaya-lockfree шард-ридеры поверх entity-шардов (broadphase read-path)

Агент: TASK-459-L11 · слот L11 (закон 11 безумие, свободен с ×457) · подсистема: broadphase (Moonrise EntityLookup/ChunkEntitySlices read-сайты) · цель: lock-free конкурентное чтение шард-таблиц broadphase при тике.

## 0. Отправная точка (банк ×459)
- Целевой лейн: **broadphase 9.36%** на монстр-ноге chk-14 (norm +21.7 @ runner 8687055, NCDFE=0, pop VALID). Лейн уже срезан носителем с 15.66% базлайна (−6.30пп) — работаем по тонкому остатку.
- Пара chk-14: leg +21.7 vs a26 +12.4@8671791 (Δ15k pair-fresh) = **+9.3**; дефицит до ≥+20 = **+10.7пп** на ноге (закон 13a: суб-бар → CLIMB).

## 1. Декомпозиция лейна (числа сэмплов, лист-ранжирование)

### Профили (cpu-collapsed пуржнуты по дисковому закону — числа из BOTTLENECKS_3/ABSORB, wall-листья сверены по wall-collapsed.txt)

| ран | runner | norm | CPU self | broadphase лейн | сэмплов лейна |
|---|---|---|---|---|---|
| chkmono457-14 (монстр) | 8687055 | +21.7 | 103062 | **9.36%** | ≈9647 |
| chkmono457-16 | 6951662 | +12.7 | 104735 | 9.87% | ≈10338 |
| chkmono457-11 (депресс, Full=9) | 7040413 | −3.6 | 104163 | 8.82% | ≈9187 |
| anchor458-33 (ваниль-класс) | 6973621 | −11.8 | 115503 | 14.68% | ≈16956 |

Лист-ранжирование broadphase-критичных фреймов chk-14 (self-сэмплы / % от 103062):
1. `ChunkEntitySlices$EntityCollectionBySection.getEntities` — 1120 / 1.1% (секционный скан, Storage-массив с null-дырами)
2. `EntityLookup.getHardCollidingEntities` — 926 / 0.9% (region-grid обход 32×32 чанков)
3. `AABB.intersects` — 871 / 0.8% (6-compare на кандидата)
4. `vtable stub` — 2667 / 2.6% (включая virtual `Entity.getBoundingBox()` на каждого кандидата)
5. `ConcurrentHashMap.get` — 992 / 1.0% (entityById/UUID/region-смешанная доля)
6. `VarHandleReferences$FieldInstanceReadOnly.getVolatile` — 1359 / 1.3% (entity-поля, acquire-фенс на чтении)
7. `HashMap.getNode` — 1459 / 1.4% (JDK collections, broadphase-доля)
8. `CollideBatchOps.blockCollisions/scanSectionVanilla` — 822+639 / 1.4% (voxel-хвост collision-плоскости — НЕ шард-ридеры)
9. Wall-проверка: libc 51969/81.6% (soak-сон), broadphase-листья в wall <0.2% — лейн чисто CPU-bound.
10. Alloc-маркер синхронизации: `ReentrantAreaLock$Node` 37 байт / 0.8% alloc-событий — area-lock уже виден в экономике аллокаций.

Разбиение лейна 9.36% (9647 сэмплов) на **reader-core** (шард-ридеры, наша цель) и voxel-хвост:
- reader-core = getEntities 1120 + getHardColliding 926 + intersects 871 + доля CHM.get ≈500 (50%) + доля VarHandle ≈400 (30%) + доля getNode ≈290 (20%) ≈ **4107 сэмплов ≈ 4.0% CPU (43% лейна)**.
- voxel/upstream-хвост ≈ 5540 сэмплов (CollisionUtil/getCollisionsForBlocksOrWorldBorder/performCollisions-кадры) — вне компетенции шард-ридеров (это swar/colpush-плоскость).

## 2. javap-контракты (5, канон round-396-a/patched-kernel.jar; javap -p -c JDK-21)

1. **`ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity,AABB,List,Predicate)`**: `getfield count` (plain int, НЕ volatile) → `ifne` early-out; y-диапазон = `Mth.floor(minY−2.0)>>4` clamp [minSection,maxSection] … `Mth.floor(maxY+2.0)>>4` clamp; по секциям `entitiesBySection[idx]` → `BasicEntityList.storage` скан `0..min(storage.length, size)` c null-skip + `if_acmpeq` self-skip → `Entity.getBoundingBox()` (virtual!) → `AABB.intersects` → predicate → `List.add`. Порядок выдачи: секции y-возрастание, внутри секции — индексный порядок storage.
2. **`EntityLookup.getHardCollidingEntities(Entity,AABB,List,Predicate)`**: чанк-диапазон `floor(minX−2)>>4 .. floor(maxX+2)>>4` (аналогично Z); **регион-грид REGION_SHIFT=5** (32×32 чанка); `getRegion(cx,cz)` через `SWMRLong2ObjectHashTable`; гейт `FullChunkStatus.FULL.isOrAfter` → `ChunkEntitySlices.getHardCollidingEntities`. Порядок обхода: **z-outer/x-inner** строки региона.
3. **`ChunkEntitySlices$BasicEntityList.add/remove`**: add = append в `storage[size++]` (grow ×2 от 4); **remove = shift-left `System.arraycopy` + null-хвост** (порядок сохраняется, дыр нет, но unlock-ридер видит сдвиг → пропуск/дубль элемента = JMM-гонка).
4. **`EntitySectionStorage.forEachAccessibleNonEmptySection` (ваниль-оригинал)**: X-секции outer, `sectionIds` **LongSortedSet.subSet** (sorted packed order), расширение ±2.0 блока по X/Z, **minY−4.0 / maxY+0.0** по Y (≠ moonrise −2.0/+2.0!); `EntitySection.getEntities` = `ClassInstanceMultiMap` итератор. Вывод: ваниль и moonrise дают разный порядок кандидатов → паритет-оракул меряем против **текущего кернела** (moonrise-порядок), не против абстрактной ванили.
5. **`EntityLookup` (класс)**: регионы `SWMRLong2ObjectHashTable<ChunkSlicesRegion>` (Single-Writer-Multi-Reader), `entityById` = `ConcurrentLong2ReferenceChainedHashTable`, `entityByUUID` = CHM; `checkThread(Entity,String)`/`checkThread(int,int,String)` абстрактные — ServerEntityLookup бросает на off-thread → **чтение легально только на тик-треде-владельце региона**; `addChunk/removeChunk` synchronized. Рядом в профиле: `ReentrantAreaLock$Node` alloc — фенс-налог текущей модели.

**Что мешает конкурентному чтению (именно мутации при тике):**
- (i) plain `count`/`size`/`storage` без фенсов: stale-count==0 → пропущенные сущности; shift-left remove → сдвиг окна итератора (пропуск/дубль);
- (ii) `checkThread`-asserts: прямой off-thread вызов = throw — конкурентный ридер невозможен без переписывания входа;
- (iii) ReentrantAreaLock: читатель обязан держать лок региона (fair-очередь, Node-alloc) — сериализация ридеров с writers;
- (iv) virtual `getBoundingBox()` + CHM/entityById хопы в горячем цикле;
- (v) ваниль-путь вообще не потокобезопасен (LongSortedSet/ClassInstanceMultiMap — однопоточные структуры).

## 3. Дизайн: papaya-стиль epoch-ридеры (rс https://github.com/papaya-dynapapaya — lockfree-таблицы)

Новый Ops-класс: **`net/minecraft/world/entity/PapayaReadOps`** (шард = ChunkSlicesRegion-эквивалент 32×32).
- **Writer** (тик-тред-владелец, существующие add/remove/move хуки): в фазе конца тика (batch-амортизация, roar-drain-паттерн) — компактный снимок секции: `Ent[]` без null-дыр + flat AABB `double[6n]`; публикация `AtomicReferenceArray.set` (release); `epoch.incrementAndGet()` odd=building → even=stable.
- **Reader** (любой тред): `epoch.get()` (acquire) → odd/изменённый → fail-closed fallback в lock-ваниль; stable → обход CSR: z-outer/x-inner, y-asc, плоский AABB-префильтр (primitive loads, без virtual getBoundingBox), `VarHandle.getAcquire` на ref — **порядок выдачи бит-в-байт = moonrise (z,x,y-asc,storage-idx)**; повторный `epoch.get()` ≠ e0 → `out.clear()` + fallback (никаких частичных публикаций).
- Что снимается с read-сайта: (i) checkThread-throw путь, (ii) area-lock acquire/release, (iii) null-дыры и `min(storage.length,size)`, (iv) virtual AABB-феч, (v) CHM/entityById-хопы (id→ref резолвится один раз на rebuild).
- Fail-closed: любая аномалия (epoch-drift, OOB, NCDFE) → авто-disarm на ваниль-путь; `threw=0` обязательный гейт.

## 4. Офлайн-эксперимент (закон 16: lockstep где возможно) — PapayaReadHarness.java, JDK-21, ×3 рана

| гейт | результат (×3) |
|---|---|
| G2a frozen-state lockstep паритет (epoch vs locked-vanilla, 300 рандом-боксов) | **0/300 mismatches ×3 → BIT-IN-BYTE PASS** (порядок z,x,y-asc,idx воспроизведён) |
| G2b race-demo unlock-ридер vs shift-left мутатор | 0/400 аномалий ×3 — **INCONCLUSIVE**: окно JMM-гонки узкое, практический репрод на этом руннере не пойман; аргумент за epoch-ридеров = снятие фенс-налога, не наблюдаемая коррупция (честная отрицательная находка) |
| G2c epoch-ридеры под гонкой (rebuild каждые 8 мутаций) | anomalies 0/400 ×3, fallbacks 316-338/400 → **FAIL-CLOSED PASS**: при drift читатель молча уходит в lock-ваниль, частичных результатов нет |
| G2-perf (614400 сущностей, 24 строки-окна, 5000 запросов) | hits идентичны 99.7M/99.7M (set-паритет); locked-vanilla 538-587 µs/q vs epoch 434-738 µs/q → **speedup 0.73-1.35×, медиана ≈1.21×** |
| rebuild-стоимость | 24×4 секций/шард — амортизируется фазой тика; при rebuild-шторме (каждые 8 мутаций) fallbacks 80% — в проде rebuild per-tick-phase, fallback-доля ожидается ≪1% (гейт G1-эффект-маркер) |

Честный вывод микро: скан-ядро ускоряется лишь ~1.2× (AABB-математика доминирует); главный съедаемый кадр — lock/virtual/lookup-надбавка, видная в leaf-фреймах 5-6.

## 5. CAPTURE-МАТЕМ (lane% × захват% = Δ; потолок = lane% × 100%)
- reader-core = **4.0% CPU** (4107/103062; см. §1), лейн целиком = 9.36% (потолок лейна +9.36пп, но 57% лейна = voxel-хвост — шард-ридерам недостижим).
- Захват reader-core: микро 1.21× → −17% скан-ядра (~350) + lock/virtual/lookup-кадры (~500-800) = захват **20-35%** → Δ = 4.0% × 0.20-0.35 = **+0.8..+1.4пп** (агресс +2.0 при полном съеде lookup-кадров).
- В терминах лейна: 9.36% × захват 8-15% = Δ **+0.8..+1.4пп**; потолок компоненты = 9.36% × 43% (reader-core доля) = **+4.1пп**; абсолютный потолок +9.36пп (недостижим без voxel-плоскости).
- Компо-лестница chk-14 (+21.7, дефицит пары +10.7пп): P31 INSIDE-BATCH (+5-8) ⊕ P32+P36 SNAP (+1.5-2.5) ⊕ **papaya-ридеры (+0.8-1.4)** → нога 29.0..33.6 → пара с a26 +12.4 = **16.6..21.2** → ≥+20 в верхней половине; papaya = третий swing-слой, закрывающий дефицит при P31≥6 и P32/P36≥1.9. Сам по себе НЕ pair-maker (≤4.1пп потолок < 10.7пп дефицита).

## 6. PREREGISTERED ГЕЙТЫ G1-G6 (CI-нога `round-459-lab-11`, носитель d73758a-класс)
1. **G1 ARM/эффект**: boot-маркер `papaya_reads ARMED` + счётчики-эффекты (epochStableReads>0, rebuilds>0, fallbacks>0, javap flat==nested 10/10 EQUAL после вайринга — спящий гейт = placebo ×425/roar-2) + лейн-проверка broadphase ↓ от 9.36%.
2. **G2 lockstep бит-в-байт**: emission id-sequence == кернел-порядок (z,x,y-asc,storage-idx); офлайн 0/300 ×3 PASS (сделано); CI-ценз: 1000 запросов lever-on/off — идентичные последовательности.
3. **G3 GC**: young ≤ 128+10%, Full ≤ 9 (chk-14 справка); rebuild-alloc ≤ +0.5MB/s (снимки секций — ephemeral young).
4. **G4 популяция-паритет**: pop 140-165k; churn band (chk-14: delta 5499, 3.6%).
5. **G5 TPS-бар**: банк v4 norm = median/TPS_exp − 1; **Δ ≥ +0.8пп** median min-of-3, band 6.0-9.5M, ре-ролл ≤2, депресс-окна мимо (Full=9 кластеры).
6. **G6 fail-closed**: NCDFE=0 (T1), threw=0, AIOOBE=0, selfTest==true; epoch-drift/OOB → авто-fallback ваниль; anything else = REFUTED.

## 7. NCDFE-канон для PapayaReadOps (обязательный)
EARLY-define в раннем arm-хуке ДО первого broadphase-запроса (прецедент EntityGoalQueryOps @ MobPushOps.pushables:467; коммиты d73758a3 chunkmono / 5ecd841a poi / 9d71b461 eqsnap2), mirror-drift игла в каждом ране; гейт вердикта: **T1 NCDFE=0** — иначе DELIVERY-FAIL, пара аннулируется (отравленные +20.1/+41.3 ×456).

## 8. Источники (5, по памяти)
1. **Moonrise Optimizations** (Spottedleaf) — ChunkEntitySlices/EntityLookup/SWMRLong2ObjectHashTable/ReentrantAreaLock: single-writer-per-region модель, checkThread-контракты (взято: javap-факты read-сайтов и threading-модель).
2. **C2ME** (ishland) — concurrentutil + параллельный entity-tick: vanilla-parity границы конкурентности (взято: SWMR-паттерн и правило «reader = владелец или снапшот»).
3. **Lithium** (jellysquid) — entity collision секционные кэши + плоские AABB-сравнения (взято: flat-prefilter вместо virtual getBoundingBox).
4. **papaya-dynapapaya** (Papaya fork) — lock-free таблицы/epoch-реименование реестров (взято: epoch-ридер-протокол H05, seqlock-семантика odd/even).
5. **Pufferfish/concurrentutil** — off-thread read прецеденты и ConcurrentLong2ReferenceChainedHashTable-экономика (взято: заменяемые CHM-хопы в read-пути).

## 9. Вердикт-число и следующий шаг
- **lane 9.36% × захват 8-15% → Δ +0.8..+1.4пп; потолок +9.36пп (реалистичный компонент-потолок +4.1пп)**.
- papaya-ридеры = swing-слой климба chk-14, не pair-maker. Следующий шаг: вайринг PapayaReadOps EARLY-define поверх d73758a-носителя (после P31/P32+P36), прегист G1-G6 + NCDFE T1=0, CI-нога min-of-3 в band 6.0-9.5M; ретро-гейт: fallbacks <1% и лейн −0.8пп иначе REFUTED.
