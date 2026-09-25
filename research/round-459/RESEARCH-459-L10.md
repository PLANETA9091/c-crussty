# RESEARCH-459-L10 — P24 «Безумие»: noise-octave scratch-pool (GC-debt carrier) — PARK с числами
Агент: TASK-459-L10 (ЛАБ, тик 459, v18.3 законы 13-16, WILD-слот закона 11).
Подсистема: PerlinNoise/NormalNoise октав-циклы — thread-лок пул флет-аккумуляторов (P24), гипотеза слота: «alloc-давление шума в worldgen-фазе бустит Full-частоту на soak-хвосте» (Full=9-10 на ногах vs банк 7).
Ген-лейн инертен на soak ×421-C (0.0%) — единственная заявленная ценность P24 = GC-debt relief carrier. Проверяем именно GC-механику.

## 1. Гипотеза слота (14a: гипотеза-дельта ДО анализа)
- H10: шум аллоцирует в октав-цикле → молодое/старое давление в soak-хвосте → Full GC чаще (9-10 vs банк 7) → STW-кластер душит ноги.
- Проверка: (i) декомпозиция Full GC по причинам (gc.log-разбор в BOTTLENECKS_3 4 прогонов), (ii) alloc-профили: есть ли noise-фреймы в alloc-окне, (iii) javap-контракты октав-циклов: аллоцируют ли PerlinNoise/NormalNoise/ImprovedNoise per-call.
- Прогноз ДО javap (честно): если Full GC = Allocation Failure-класс → пул мог бы дать 1-2 Full → по slope L05 (−4.96пп/с STW) до +10-15пп. Если Full = CodeCache/Metadata — гипотеза мертва немедленно.

## 2. Профильная декомпозиция (4 прогона: монстр / нейтрал / депресс / RED-якорь)
Все цифры из BOTTLENECKS_3.md/ABSORB.md профилей research/gc-recon-2026-09-19/ (сэмплы = async-profiler, прогоны чкmono457-14/-16/-11 + anchor458-33):

| прогон | runner | norm | CPU-сэмплов | noise-lane self | young GC | Full GC | STW total | avg/max pause |
|---|---|---|---|---|---|---|---|---|
| chkmono457-14 (монстр) | 8687055 | +21.7 | 103062 | 25 = 0.024% | 128 (118 AllocFailure) | 9 (5 CodeCache + 4 Metadata) | 17.66s | 129/1955ms |
| chkmono457-16 | 6951662 | +12.7 | 104735 | 26 = 0.025% | 102 (93 AllocFailure) | 9 (5 + 4) | 20.53s | 185/3010ms |
| chkmono457-11 (депресс) | 7040413 | −3.6 | 104163 | 21 = 0.020% | 96 (87 AllocFailure) | 9 (5 + 4) | 22.1s | 211/2789ms |
| anchor458-33 (RED) | 6973621 | −11.8 | 115503 | 30 = 0.026% | 106 (96 AllocFailure) | 9 (5 + 4) | 25.19s | 219/2537ms |
| банк-справка | — | — | 115655 (базлайн) | — | — | 7 | 18.8s | 162/2400ms |

### Лист-ранжирование ключевых чисел (≥5, закон 14d)
1. **Full GC = 9 на ВСЕХ четырёх прогонах, и 0 из 9 — Allocation Failure**: ровно 5×Full(CodeCache GC Threshold) + 4×Full(Metadata GC Threshold) в каждом прогоне, включая RED-якорь и монстра. Корреляция Full↔norm = константа/константа → Full=9 НЕ свойство ног и НЕ alloc-следствие; «банк 7» — конфиг-артефакт справки (иной класс-сет/JIT-объём), а не heap-давление.
2. **Noise CPU-лейн на soak = 21-30 self-сэмпла из 103062-115503 = 0.020-0.026%** (4/4 прогона; wall-профили 1/63661 и 0) — ниже любого измеримого порога (у chk-14 PSCardTable 1462 = 1.4%, noise в 58× меньше).
3. **Alloc-окно (последние 60s soak): 0 noise-фреймов октав в top-20** всех прогонов; топ-сайты = Vec3 421-582 (13.9-14.9%), AABB 389-566 (12.8-14.5%), char[] 307-445, byte[] 200-246, BlockPos/MutableBlockPos ~90-149, ArrayList/long[]/Object[] ~90-138. Единственные «Noise»-строки в alloc-collapsed = `NormalNoiseBatchOps$Reaper.run → AQS$ConditionNode_[i] 10-11 scaled units` из 3029-3896 всего (= 0.28-0.36%) — это **инфра-само-cost Reaper-потока носителя noisesimd (agent-D), НЕ октавы**.
4. **Young GC = 87-128 AllocFailure** — единственный alloc-зависимый GC-класс; его давление создают entity-фазы (Vec3/AABB-чурн региона-плейна), варьирует 96→128 вместе с силой ноги (быстрее тик → больше итераций/чурна), а не с шумом.
5. **STW-кластер co-рана сессии-B**: STW 17.66→25.19s анти-коррелирует с norm (L05: r≈−0.93, slope −4.96пп/с) — это РЕАЛЬНЫЙ фактор депрессий, но его драйверы — Full CodeCache/Metadata-хвосты (max pause 1955-3010ms) и young-чурн entity-плейна; шум в обеих стопках отсутствует.
6. Wall-присутствие infra-потоков noisesimd-носителя: Census/Reaper sleep-строки 432+769+1201 wall-сэмплов при CPU=0 — wall-профиль обманчив, не считать за cost (урок для T5-гейтов).

### Вывод по гипотезе H10
H10 **REFUTED на данных**: Full-частота (9=5CC+4MD) не зависит от alloc-давления вообще (0 AllocFailure-Full), а noise-alloc в soak-окне = 0 наблюдаемых октавных фреймов. Пул нечему разгружать.

## 3. javap-контракты (jar: research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar, javap -p -c, JDK21)
- **К1 PerlinNoise.getValue(DDDD DZ) — октав-цикл (байткоды 18-136)**: аккумулятор = scalar-локал `dstore 12 / dload 12`; цикл по `noiseLevels[]` (aaload) × `amplitudes.getDouble(i)` × `ImprovedNoise.noise(DDDDD)`; **0 опкодов `new`** в цикле. Аккумулятор октав УЖЕ живёт в стековом слоте JVM — пулировать нечего.
- **К2 NormalNoise.getValue(DDD)**: 3 scalar-умножения на 1.0181268882175227 + два вызова PerlinNoise.getValue + valueFactor; **0 опкодов `new`**. «Флет-аккумулятор» NormalNoise = один double-локал.
- **К3 ImprovedNoise.noise(DDDDD)+sampleAndLerp**: чистые double-локалы (xo/yo/zo сдвиги, Mth.floor, lerp-цепочка), **0 `new`**; единственный `newarray byte` — в ctor (перестановочная таблица, boot-фаза).
- **К4 (дополнительный) PerlinNoise ctor + NoiseChunk$NoiseInterpolator**: аллокации шума (`new ImprovedNoise` на октаву + `makeConcatWithConstants`-строка, DoubleArrayList, `NoiseInterpolator` — 1 `new` на класс) сосредоточены в **конструкторской/boot-фазе** (создание NoiseRouter до soak) — вне профилируемых окон и вне досягаемости per-call пула.
Итог javap: **октав-циклы ванили аллокат-free по построению** — P24 решает несуществующую проблему; любой пул = чистый оверхед (TLS-lookup ≥ cost сохранённого аллока = ноль).

## 4. Внешние источники (по памяти, ≥3)
1. **Noisium** (Steveplays28, MC 1.20-1.21): оптимизирует именно **compute октав-циклов** (переписанный sampling, развёрнутый lerp, меньше операций на октаву), а не аллокацию — принятая сообществом ось для этого кода совпадает с нашей javap-правдой: аллокаций там нет. Что взято: подтверждение оси (CPU-in-octaves, не pooling) → для c-crussty остаётся только bulk-JNI/SIMD-вариант носителя noisesimd (cmp457_noisesimd @c8156a69, agent-D), P25 2D-router cache отдельно.
2. **C2ME** (0x3C50, fabric): параллельный worldgen на executor-воркерах; документированная тонкость — инстансы шума не thread-safe → per-thread экземпляры/сэмплеры, а не пулы аккумуляторов. Что взято: если когда-нибудь revived — только thread-local на chunk-system воркерах (Moonrise-паттерн), что и было замыслом P24; ваниль-путь аллокат-free делает это moot.
3. **Lithium** (CaffeineMC): в модульном списке НЕТ noise/octave-модуля при агрессивном покрытии мира/блоков — negative precedent: ROI октав-микрооптимизаций в general-purpose моде признан недостаточным (worldgen-лейн трогали только структуры/BlockPos).
4. **Moonrise** (Spottedleaf) + MC-310372: chunk-system воркеры выносят gen/тик с main; thread-local scratch — их каноническая механика (ConcurrentUtil) — механический шаблон для P24, но с нулевой базой аллокаций применение пустое; живой закон-8 сайт остаётся chunk-tick eligibility (ID-P22).
5. **Paper/Pufferfish GC-док**: Full GC по CodeCache/Metadata — известный JVM-феномен (code-cache flushing при JIT-объёме), лечится JVM-флагами/класс-сетом, не heap-диетой → прямо объясняет наш «Full=9 vs банк 7».

## 5. CAPTURE-МАТЕМ (закон 13a/16)
- **Прямой лейн**: lane% = 0.020-0.026% CPU; Δ_прям = lane × захват ≤ **+0.026пп** (при 1пп lane ≈ 1.26пп norm по chk-14-эмпирике L05 → ≤ +0.033пп norm). Потолок прямого лейна = +0.026пп. К бару +20 — в 769 раз ниже.
- **GC-путь (заявленная ценность P24)**: Full-компонента = 5 CodeCache + 4 Metadata, **0×AllocFailure → пул не может снять НИ ОДНОГО Full** (максимум гипотезы слота = мёртв). Young-компонента: база захвата = наблюдаемый noise-alloc 0.28-0.36% scaled (и это Reaper-инфра чужого носителя, не октавы). Математический СУПЕРМАКСУМ (захват 100% всего observable noise-alloc): 0.36% × 22.1s STW = 0.08s × 4.96пп/с = **+0.39пп**. Реальный захват октавным пулом = 0% (javap К1-К3: аллокаций в цикле нет) → **Δ = +0.00пп**.
- **Потолок P24 честно**: +0.39пп теоретический supermax / +0.026пп прямой — оба << +20 → REFUTED_CENS по потолочной математике (закон 13a): даже 100% захват всех наблюдаемых лейнов не достигает бара на 51×.
- Что могло бы перерегистрировать гипотезу: alloc-профиль с noise ≥5% alloc-байт в soak-окне (сейчас 0.0%) или Full(AllocFailure) ≥1 (сейчас 0/9×4).

## 6. Preregistered гейты G1-G6 (если кто-то диспатчит вопреки PARK)
- **G1 ARM/эффект**: run-env `CRUSSTY_NOISE_POOL=1` + stdout-маркер `NoisePoolOps ARMED gets=N>0` до пурджа; disarm fail-closed при selfTest=false; NCDFE=0 — T1-канон (урок-408: спящий гейт = DELIVERY-FAIL).
- **G2 lockstep бит-в-байт**: оракул = `PerlinNoise.parityConfigString`/`NormalNoise.parityConfigString` (контракты уже в jar) + бит-в-байт хеш значений шума до/после на lockstep-харнессе (strictfp-семантика, запрещены FMA/reassociation/кэш-агрегация аккумулятора).
- **G3 young/Full GC**: young ≤128, Full ≤9, STW ≤19s; эффект-маркер = Δyoung ≥5% вниз И ΔFull ≥1 — иначе captures=0 → авто-REFUTED без ре-роллов.
- **G4 популяция-паритет**: pop 140-165k, churn ACTIVE, fake_players 4/4, spawnable 289.
- **G5 TPS-бар vs банк v4**: norm ≥+20 (TPS_exp(runner)); при потолке +0.39пп нога априори суб-бар → диспатч запрещён без нового лейна.
- **G6 band/fail-closed**: band 6.0-9.5M runner_cpu_index, ре-ролл ≤2; любое AIOOBE/threw=1 → REFUTED.

## 7. ВЕРДИКТ: **PARK (числами)** — P24 noise-octave scratch-pool
- Прямой лейн 0.020-0.026% CPU; GC-механика Full не alloc-зависима (0/9 AllocFailure × 4 прогона); октав-циклы аллокат-free (javap 0×`new`, контракты К1-К3); supermax +0.39пп << +20.
- Ценность WILD-слота исполнена: гипотеза слота убита числами ДО диспатча (zero-CI-cost), знание зафиксировано в LEDGER (закон 14c).
- Остаток живого в подсистеме noise: только носитель cmp457_noisesimd @c8156a69 (SIMD/bulk-JNI compute-ось, P25 2D-router cache — отдельная гипотеза, не GC).

## 8. Следующий шаг
PARK закрыт; в RE-ГРАЙН-цикле (закон 15a) не ре-роллить P24. Если tик ищет GC-носители — молодое давление создают Vec3/AABB entity-чурна (regional tick: 14.9%+14.5% alloc top-сайтов) → там и только там scratch-pool может иметь базу; для noise-подсистемы следующий кандидат = P25 (2D-router cache) при появлении worldgen-фазы в фикстуре (boot/forceload-фокус, не soak).
