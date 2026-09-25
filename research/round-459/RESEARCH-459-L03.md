# RESEARCH-459-L03 — POI-подсистема (носитель cmp456_poi @5ecd841a) — тик 459, v18.3 законы 13-16

Слот L03: разбор дисперсии ног +16.9..−0.5, гипотезы-дельты для ре-роллов 16-18, потолок пары (якорь ≤−3.1 в [8907260,9007260]).

## 1. Инвентаризация POI-ранов (15 ранов, 17 артефакт-директорий round-poi*/round-round-456b-poi*)

| нога | run | ветка/head | runner | median | TPS_exp | norm | вердикт | band |
|---|---|---|---|---|---|---|---|---|
| poi457-13 | 36146420766 | round-456b-poi-13/5ecd841 | 6660866 | 2.50 | 2.20 | **+13.6** | GREEN | OK |
| poi457-5r2 | 36132360006 | round-456b-poi-5r2/5ecd841 | 6733336 | 2.10 | 2.22 | **−5.3** | RED | OK |
| poi456-1 | 36121914548 | round-456b-poi-1/084368d | 6765332 | 2.60 | 2.22 | **+16.9** | GREEN-CAND | OK |
| poi457-11 | 36144116531 | round-456b-poi-11/5ecd841 | 6953086 | 2.35 | 2.26 | **+3.8** | PARITY/LOW | OK |
| poi457-6 | 36131797497 | round-456b-poi-6/5ecd841 | 6997574 | 2.55 | 2.27 | **+12.2** | GREEN-CAND | OK |
| poi457-9 | 36134005127 | round-456b-poi-9/5ecd841 | 7054948 | 2.40 | 2.28 | **+5.1** | GREEN-CAND | OK |
| poi457-10 | 36144102915 | round-456b-poi-10/5ecd841 | 7143835 | 2.60 | 2.30 | **+12.9** | GREEN-CAND | OK |
| poi456-3 | 36126727936 | round-456b-poi-3/5ecd841 | 7181470 | 2.30 | 2.31 | **−0.5** | PARITY/LOW | OK |
| poi457-7 | 36131807435 | round-456b-poi-7/5ecd841 | 7523645 | 2.60 | 2.38 | **+9.1** | GREEN-CAND | OK |
| poi456-2 | 36122777112 | round-456b-poi-2/084368d | 8791818 | 3.10 | 2.65 | +17.0 | **DELIVERY-FAIL (NCDFE=6014)** | OK |
| poi456-4 | 36126738321 | round-456b-poi-4/5ecd841 | 8957260 | 3.10 | 2.69 | **+15.4** | GREEN-CAND | OK |
| poi457-12 | 36146408002 | round-456b-poi-12/5ecd841 | 9830285 | 3.40 | 2.87 | +18.5 | **BAND-OUT** (9.83M>9.5M) | OUT |
| poi457-8 | 36132385298 | round-456b-poi-8/5ecd841 | 10425941 | 3.40 | 2.99 | +13.5 | **BAND-OUT** (10.43M) | OUT |
| poi457-14 | 36151076473 | round-456b-poi-14/5ecd841a | 7223047 | 1.90 | 2.32 | **−18.1** | VALID (compose-стек) | OK |
| poi457-15 | 36151088554 | round-456b-poi-15/5ecd841a | 8833092 | 2.80 | 2.66 | **+5.3** | VALID (compose-стек) | OK |

Чистые in-band ноги без compose (n=10): mean +8.3, median +10.6, std 7.3, размах −5.3..+16.9.

## 2. Почему дисперсия ног −5.3..+18.5 на ОДНОМ коде (лейн-разбор)

### 2.1 Лейны КОНСТАНТНЫ — дисперсия не лейновая
T5-лейны по 11 чистым ногам (ABSORB, базлайн 115655 сэмплов):
- inside_volatile: 15.71..17.91% (макс 17.91 у середнячка +9.1; минимум 15.71 у +15.4) — RANG не коррелирует с norm;
- broadphase: 8.93..10.92%; nav_ai: 2.94..3.69%; java_util: 8.29..9.80%; fluid: 15.70..18.79%; fastutil: 5.94..7.41%; paletted: 5.14..6.73%; items: 0.00% везде (era-юнион).
Профили wall-collapsed (poi457-12 +18.5 vs poi457-13 +13.6): TOP-листья бит-похожи — libc 81.45%/80.97%, clock_nanosleep 7.52%/7.49%, PalettedContainer.get 0.21%/0.20%. Полная wall-атрибуция (мой скрипт, 63652/63659 сэмплов) не даёт ни одного лейна, различающего лучшую и худшую ногу >0.3пп.
Вывод: **дисперсия норм не объясняется составом лейнов** — код работает одинаково, различается среда.

### 2.2 Квантизация медианы = ±1.7..2.3пп шума
median из 5-6 поллов с сеткой 0.1 TPS → полушаг 0.05 TPS. При TPS_exp 2.20 (runner 6.66M) это ±2.3пп norm, при 2.87 (9.83M) ±1.7пп. Одного шага медианы хватает, чтобы poi456-1 +16.9 стал +12.4 (median 2.50 вместо 2.60).

### 2.3 Раннер-лотерея и GC
- r(runner, norm) по 10 чистым in-band = +0.30 (слабый позитив): класс <8.79M (n=9) mean +7.5±7.3; класс ≥8.79M: единственная валидная poi456-4 +15.4, band-out подтверждает +18.5@9.83M и +13.5@10.43M.
- GC: young 102..130, Full=9-10 У ВСЕХ (банк 7) — системное, не дискриминатор; max-pause 2258..3050ms; total 19.4..24.1s без корреляции с norm (лучшая нога +16.9: 19.8s/2558ms; худшая −5.3: 20.7s/2782ms).
- Кросс-семейный контроль: chkmono457-16 +12.7@6951662 vs poi457-11 +3.8@6953086 (Δrunner=1.4k!) → на одном ранере chunkmono сильнее poi на 8.9пп; но poi457-6 +12.2@6997574 сравнивает счёт. Каждая нога = один лотерейный билет (подтверждение BOTTLENECK ×459).

### 2.4 Вывод по дисперсии
Дисперсия −5.3..+18.5 = (а) квантизация медианы ±2пп, (б) раннер-фикстура (лотерея позиций), (в) GC-кластерные окна (Full=9-10 > банк 7 у всех). Лейны НЕ виновны. Следствие (закон 14a): любой вердикт poi-ноги ниже +12.9 недостоверны без min-of-3; ре-роллы обязаны нести новую гипотезу, а не «повторить».

## 3. javap-контракты (3, jar: round-poi457-14/patched-kernel.jar 29.4MB + блоб PoiOps из коммита 084368d7; javap = /home/z/tools/jdk-21.0.12.1+1/bin/javap)

### 3.1 PoiOps.updatePoiGate (блоб-класс, 7356 байт) — fast-path контракт
```
public static void updatePoiGate(Level, BlockPos, BlockState, BlockState):
  0: getstatic ENABLED:Z; ifeq 89            // выключен → vanilla
  6: getstatic broken:Z; ifne 89             // fail-closed
  12: getstatic MASK:[J; ifnull→ensureMask    // DOD-маска stateId→isPoi (poiBindMask)
  41-57: BLOCK_STATE_REGISTRY.getId(old/new) ×2
  59-79: isPoi(MASK,old)==false && isPoi(MASK,new)==false → return   // fast path: vanilla skip
  80-86: appendEvent(...)                    // телеметрия батча
  89-93: invokevirtual Level.updatePOIOnBlockStateChange(pos,old,new)  // SLOW path = FULL vanilla
```
poiTickGate(PoiManager, BooleanSupplier): maybeEpoch() (один bulk-JNI poiEpoch за тик) + vanilla pm.tick. Нативы: poiProbe/poiBindMask/poiEpoch; PROBE_MAGIC 0x5049 "PI"; ERR_STRUCT=-1/ERR_RANGE=-2.Zero per-block JNI (закон 6).

### 3.2 Сайты-ретаргеты в ядре — свера единственности (1/1)
- Level.notifyAndUpdatePhysics: `256: invokevirtual #1608 updatePOIOnBlockStateChange` — ровно 1 вхождение в Level.class ✓ (rust-константа FROM_POI_UPDATE: имя+desc совпадают байт-в-байт).
- ChunkMap.tick(BooleanSupplier): `13: getfield poiManager; 18: invokevirtual #786 PoiManager.tick` внутри profiler-секции "poi" — ровно 1 ✓ (FROM_POI_TICK desc (Ljava/util/function/BooleanSupplier;)V ✓).
- PoiOps.class в kernel jar ОТСУТСТВУЕТ (инжект через OPS_BYTES при загрузке) — это не дефект: ретаргет на лету через classfile::retarget_virtual_to_static.

### 3.3 PoiManager (ядро 1.21.10 + moonrise) — что уже сделано до нас
PoiManager extends SectionStorage<PoiSection,PoiSection$Packed> implements ChunkSystemPoiManager (moonrise): Delayed26WayDistancePropagator3D villageDistanceTracker, moonrise$loadInPoiChunk (POI грузится chunk-system'ом off-main), LongSet loadedChunks. API: get/getOrLoad/getOrCreate, add/remove, getCountInRange, getInSquare/getInRange/getInChunk/findAll/findAllClosestFirstWithType (стримы). Т.е. IO/загрузка POI уже off-main — остаётся только CPU-диспетч (см. §4) и стрим-апи поведения AcquirePoi.

## 4. CAPTURE-МАТЕМ: POI-лейн СУХОЙ на фикстуре 150k

Замер PoiManager|PoiSection|PoiRecord|AcquirePoi|PoiCompetitor|updatePOIOnBlockStateChange|MaybeFilledPOI по collapsed:

| профиль | сэмплов | POI-сэмплы | доля |
|---|---|---|---|
| cpu-collapsed poi457-14 | 105390 | 223 | **0.21%** |
| cpu-collapsed poi457-15 | 106087 | 240 | **0.23%** |
| cpu-collapsed anchor458-36 | 116080 | 174 | **0.15%** (ваниль-уровень) |
| wall-collapsed poi457-12/13/14/15 + anchor458-33/34 | 61-64k | 2..12 | 0.00-0.02% |
| alloc-collapsed poi457-14/15, anchor458-36 | 3.5-4.0k | 2 | 0.05-0.06% |

- lane% × захват% = Δ-прогноз: весь POI-лейн = 0.15-0.23% CPU; захват fast-path уже ~100% двух сайтов (site-1/site-2) → остаток для новых poi-only рычагов ≤ 0.23%×(1−0.9) ≈ **+0.02пп**; ПОТОЛОК POI-плоскости = 0.23% × 100% = **+0.23пп** « +20пп.
- **Honest REFUTED-by-capture (закон 13a): чисто-POI вектора на этой фикстуре закрыты математикой** — даже Lithium-класс рерайт PoiManager даст ≤+0.2пп. Сила cmp456_poi НЕ в poi_plane.rs.
- Где реальная дельта: STRICT-OR widen cmp456_poi в 24 rust-файла/10 java-мостов + CARRIER_UNION_456 маркеры (chunkparse/chunksend/encode). Matched-runner пары diet-эра (cmp453_diet, POI-плоскость тоже armed в её гейтах) vs poi: n=8, Δrunner≤25k, **mean +6.1пп, median +9.9пп, размах −18.3..+18.2пп** — это верхняя оценка union-инкремента ×456-ревизии (новые блобы+widening), НЕ чистой POI-плоскости.

## 5. Compose-дилution: entity_compose поверх poi = НЕЛЬЗЯ

- poi457-15 (compose-стек: entity_compose,region_threads,inside_snap,cmp406_sscan,cmp412_b2p1,cmp401_collide,cmp420_chunk2,cmp444_chunk5,cmp437_chunk4,cmp406_aibatch,cmp421_brain,cmp420_colpush,cmp422_brain2) +5.3@8833092 vs poi456-2 чистый +17.0@8791818 (Δrunner=41k) → **дилution −11.7пп**.
- poi457-14 (тот же стек) −18.1@7223047 vs poi457-10 чистый +12.9@7143835 (Δrunner=79k) → **дилution −31.0пп** (стек ушёл в минус сам по себе).
- Alloc-профили poi-14/15 и a36 топологически одинаковы (Vec3 13-15%, AABB 12-15%, char[] ~11%, BlockPos ~7%) → дилution не аллока-природа; атрибуция по носителям требует маркеров, но менеджерское правило уже численно: compose-стек поверх cmp456_poi = −12..−31пп → исключить из poi-ре-роллов.

## 6. Гигиена банка якорей (крит-находки для вердиктов ×459)

1. **a35 (round-anchor458-35, run 36146463025, "+13.6@6660866") — ФАНТОМ**: T5/GC/median бит-идентичны poi457-13 (run 36146420766): total=104993, items 0.00%, broadphase 9.46%, GC 21.6s/191ms/2712ms. У ванильного якоря items ~29-31% (a33 29.05%, a24 29.82%) — items 0.00% на якоре невозможен. Прецедент-урок LEDGER «absorb-кэш склеивает артефакты (tag-vs-run_id)» — второй случай. **Исключить a35 из банка/окон.**
2. **a36/a37 (round-anchor458-36/37) — НЕ ванильные якоря**: T1 lever=entity_compose,region_threads, **armed=ДА**; их norms (−5.0@7204933, +0.7@7050499) содержат эффект носителей → пары с ними контаминированы (для пары надо ваниль-якорь: items ~29-31%).
3. **a24-456w2 (round-456-anchor-24, run 36116759710, head f70402e) — ЖИВОЙ ваниль-якорь в окне D**: norm −10.4@8928192, band OK, NCDFE=0, pop VALID, items 29.82% (чистая ваниль), поллов=5. Упоминаний в мерджах ×451/453/455 нет (рождён ×456w2 ПОСЛЕ ×455-мержа) → pair-fresh.

## 7. Потолок пары и реалистичность окна D [8907260,9007260]

Математика: pair = leg_norm − anchor_norm ≥ +20, Δ≤50k, pair-fresh, min-of-3.

### 7.1 Исторические якоря в окне D (4 шт.)
| якорь | runner | norm | Δк poi456-4@8957260 | pair с poi456-4 (+15.4) |
|---|---|---|---|---|
| a15 | 8914125 | +4.6 | 43k | +10.8 |
| **a24-456w2** | **8928192** | **−10.4** | **29k** | **+25.8 ≥ +20** |
| a1-456 | 8935474 | +4.4 | 22k | +11.0 |
| a8-457 | 9004607 | +3.9 | 47k | +11.5 |
(anchor-7 −0.9@8901576 — Δ=55.7k>50k, мимо). P(якорь ≤−3.1 в D) по истории = 1/4 = **25%**.

### 7.2 Глобальная якорная статистика (108 ваниль-якорей in-band 6.0-9.5M)
mean −2.2, median −1.5, std 7.4, min −25.6 (a27-457), max +12.4 (a26-457); P(norm ≤−3.1) = 43/108 = **40%**. Плотность якорей в районе окна [8.395,9.152]M = 17 ранов на 0.757M ≈ 22.5/М → в 1М-окно исторически попадает ~4 якоря, из них ~1 слабый. Якорные волны ×458/459 (a33@6973621, a36@7204933, a37@7050499) стреляли в 6.9-7.2M зону — окно D ПОКА НЕ ОБСТРЕЛЯНО.

### 7.3 Вердикт реалистичности: ОКНО РЕАЛИСТично, пара существует УЖЕ СЕГОДНЯ
- Порог «якорь ≤−3.1» (для ноги +16.9; ≤−4.6 для poi456-4 +15.4) выполняется исторически с P=25-40% на якорь-ролл в окне.
- a24-456w2 −10.4@8928192: пара с poi456-4 = **+25.8**, Δ=29k, pair-fresh ✓. Блокер только один: min-of-3 (нога парится с ≥3 якорями) — сейчас в окне 4 якоря, из них ≥+20 только с a24. Закрытие: (а) 2 ре-ролла poi-ноги у [8.93,8.98]M (min-of-3 по ПОКАЗАТЕЛЮ ноги — stability-проверка), и/или (б) 3-5 свежих ваниль-якорей на current master 6e019e41 в [8907260,9007260]: P(≥1 слабый) = 1−0.6^3..0.6^5 = 58-78% при целевом обстреле окна.
- Оговорка свежести: a24 снят на head f70402e (до paldelta cert-fix 29876077 в master) → при вердикте нужен контрольный свежий якорь рядом (дельта master-базы), пара «под подозрением» до min-of-3.

## 8. Гипотезы-дельты для ре-роллов poi-16..18 (preregistered; каждое — гипотеза/лейн/прогноз/отличие)

| # | гипотеза-дельта | лейн/таргет | прогноз | чем отличается от пред. |
|---|---|---|---|---|
| R16 | high-runner uplift: norm растёт к 9M (band-out +18.5@9.83M, +15.4@8.96M vs mid mean +7.5) | чистый poi, ранер-таргет [8.90,9.50]M | нога 15-19пп (median ~16.5±2) | первые poi-роллы, прицельно в верхний band (все прошлые ≤7.52M случайно) |
| R17 | union-isolation: дельта poi-vs-diet = юнион-расширение (chunkparse/chunksend/encode маркеры), не POI-плоскость | A/B один коммит master-блобов: lever=cmp453_diet ×3 vs cmp456_poi ×3, matched ±25k | Δpoi−diet ≥+2пп подтвердит юнион; Δ<+2 → poi-носитель субсумирован → пере-маркировка банка | первый контролируемый A/B (все 8 прошлых пар — поперёк эпох блобов) |
| R18 | pair-lock: закрить пару ≥+20 с a24-456w2 | 2 ре-ролла poi у [8.93,8.98]M + 3 свежих якоря в окно D | min-of-3 ноги ≥+13; ≥1 якорь ≤−3.1 (P=58-78%) | первый таргет-обстрел окна D (×458/459 стреляли мимо) |

Гейты G1-G6 (канон неизменен): G1 ARM+эффект-маркеры ДО пурджа (javap flat==nested 10/10 + strings-проверка блобов — против спящих гейтов ×425/roar-2); G2 lockstep бит-в-байт (fast-path PoiOps: vanilla-skip только при оба не-POI — observational no-op); G3 young/Full GC мониторинг (Full>9 = депресс-кластер, ре-ролл в окно); G4 pop 140-165k; G5 TPS-бар vs банк v4 (2.6@8551924/2.2@6653417); G6 fail-closed disarm (broken=true → чистая ваниль). Плюс: NCDFE=0 (канон EARLY-define), threw=0, AIOOBE=0, selfTest==true, items-гейт, argv-guard, NOT-A-BENCH, band 6.0-9.5M ре-ролл ≤2.

## 9. Внешние источники (по памяти, ≥3: проект + тема + что взято)

1. **Moonrise (Spottedleaf)** — chunk_system POI: PoiChunk, off-main POI load (moonrise$loadInPoiChunk), Delayed26WayDistancePropagator3D. Взято: подтверждение javap'ом, что IO/загрузка POI уже вне main-thread в нашем ядре → остаточный POI-лейн CPU-only 0.2%, что и дало honest-capture refute.
2. **Lithium (CaffeineMC)** — mixins world.poi (PointOfInterestStorage/тип-кэши, замена стримов getInSquare на прямые обходы секций). Взято: класс механики «если бы лейн был жив»; capture-матем показывает ≤+0.2пп даже при 100% — на эту фикстуру не диспатчить.
3. **Paper** — патчи кэширования POI-запросов и отложенного POI-flush на выгрузке чанков; батчинг POI-событий за тик. Взято: дизайн site-2 flush (наш poiEpoch = один bulk-JNI за тик) соответствует Paper-практике; следующий шаг для живых лейнов — batch-плоскости chunkparse/chunksend (CARRIER_UNION_456).
4. **C2ME** — многопоточность world-tick/POI-запросов поведения (AcquirePoi off-main). Взято: отклонено числами — AcquirePoi 0.01-0.02% wall (9 сэмплов на 63652 у poi457-12) — шить нечего.
5. **Баг-класс MC (villager/POI churn при unload/reload секций)** — motivating контекст для POI-тяжёлых миров; на фикстуре 150k items/husk/creeper POI-активность минимальна (0.2%) — вывод: POI-подсистема важна для ваниль-игры, но НЕ для этого бенча; потолок мерджа брать из chunk-parse/send юниона.

## 10. Что выяснено глубже вчерашнего знания (закон 14e)
1. POI-лейн на фикстуре = **0.15-0.23% CPU / 0.05-0.06% alloc** → потолок POI-плоскости +0.23пп; все +13..+18.5 ног — эффект era-юниона (widening+блобы), matched diet-vs-poi = +6.1пп mean (n=8).
2. Дисперсия ног −5.3..+18.5 на одном коде — **не лейновая** (лейны константны ±1пп) и не GC-дискриминируемая (Full=9-10 у всех); = квантизация медианы ±1.7-2.3пп + раннер-лотерея (r=+0.30) → вердикты < +12.9 требуют min-of-3.
3. **a24-456w2 −10.4@8928192 (Δ=29k, pair-fresh)** даёт пару +25.8 с poi456-4 +15.4 УЖЕ СЕЙЧАС; окно D обстреляно на 25% (1/4 слабых), глобальная P(≤−3.1)=40% (43/108 якорей).
4. Банк-гигиена: a35 — фантом-склейка (items 0.00% на «якоре», T5 бит-равен poi457-13); a36/a37 — не ваниль (entity_compose+region_threads armed=ДА).
5. entity_compose поверх cmp456_poi = дилution −11.7..−31.0пп (matched) — запретить композиции poi+compose до атрибуции по маркерам.

## 11. Следующий шаг
Диспатчи R16/R17/R18 (по одному на агента-климбера): R16 pure-poi @8.90-9.50M; R17 diet-vs-poi A/B ×3×3 на одном коммите; R18 min-of-3 ноги у 8.93-8.98M + 3 ваниль-якоря в [8907260,9007260]. Проверить pair-fresh a24-456w2 у владельца банка и удалить a35 из банка. LEDGER-строки — в LEDGER-L03.md и docs/LAB_LEDGER.md (POI-секция переписана).
