# RESEARCH-468-S62 — CHUNK-ПАЙПЛАЙН javap-КАРТА (19a): стадии загрузки/генерации/сериализации, Ops-блобы пайплайна, свободные стадии

Автор: S62 (плоский рой v20, ROUND-468, тик ~23:1x+08). База: origin/master **c1196321** (Л180).
Источники байткода: /home/z/tools/patched-kernel.jar (Purpur/Paper 1.21.10 kernel, build 2025-12-11) — тот же KERNEL_JAR, что build_456b_blobs_all.sh; javap JDK-21.0.12.1 (`javap -c -p`).
Дампы (17 классов, ~1.3 MB): /home/z/rounds/ROUND-468/s62_art/*.javap. Master не тронут.

## 1. Стадийная карта пайплайна (javap-числа: class-bytes / methods / instructions)

| стадия | класс | байт | методов | инстр | hot-метод (инстр) |
|---|---|---|---|---|---|
| LOAD-parse | storage/SerializableChunkData | 55,243 | 87 | 2,203 | read(ServerLevel,...)=**700**, write()=**69** |
| LOAD-io | storage/RegionFileStorage | 20,063 | 37 | 1,014 | read=**151**, write=**138** (off-main IO Moonrise) |
| LOAD-io | storage/RegionFile | 34,171 | 77 | 2,510 | — |
| LOAD-poi | poi/PoiManager | 37,684 | 74 | 910 | — (loadPoiChunk Paper#13713 150ms-спайк — Л179) |
| GEN-noise | levelgen/NoiseBasedChunkGenerator | 38,981 | 40 | 1,184 | λ$fillFromNoise$11=**178** |
| GEN-noise | levelgen/NoiseChunk | 21,826 | 66 | 942 | fillSlice=**91**; вызов Aquifer.computeSubstance @NoiseChunk:1106 |
| GEN-aquifer | levelgen/Aquifer$NoiseBasedAquifer | 15,191 | 60 | 1,402 | computeSubstance=**278** (C2ME #558/#557 cell-cache +3-7% c/s) |
| GEN-synth | synth/ImprovedNoise | 5,691 | 14 | 856 | noise(DDDDD)=**106** (native-noise 64k ×1.2-1.4 c/s target) |
| GEN-synth | synth/BlendedNoise / PerlinNoise / NoiseRouter | 9,400/11,030/10,143 | 30/28/39 | 437/554/**228** | NoiseRouter-DAG warm +35% (анти, TASK-100) |
| SCHED-tick | moonrise/NewChunkHolder | 66,291 | 145 | 3,931 | hot ~60 инстр (Л65); getLastChunkCompletion 3 инстр = 0.054-0.073% top-кадр ×3/3 (Л103) |
| SCHED-tick | server/ChunkMap | 62,665 | 142 | 2,136 | — |
| SCHED-tick | server/ServerChunkCache | 39,350 | 99 | 1,439 | **getChunkNow=51 инстр, moonrise$setFullChunk=19 инстр** = оба live-ретаргета ChunkSchedOps |
| SCHED-tick | server/ChunkHolder; status/{ChunkStatus,ChunkStep,ChunkPyramid} | 19,306; 8,193/7,916/8,341 | 55; 50/20/34 | 662; 272/255/228 | — |
| SEND-серализация в сеть | server/network/PlayerChunkSender → ClientboundLevelChunkWithLightPacket.write | (targets classfile.rs:9102/9204) | — | — | redirect sendChunk (cmp437_chunk4) + write (cmp444_chunk5) |

Лестница статусов (ChunkStatus/ChunkStep/ChunkPyramid = 24,450 B, 755 инстр суммарно) — оркестрация ген-лестницы, сама работа в gen-классах.

## 2. Ops-блобы пайплайна (из 75 include_bytes!-блобов src/*.rs — 13 классов в 7 модулях)

| блоб | байт | модуль/lever | стадия | статус на master c1196321 |
|---|---|---|---|---|
| **ChunkSchedOps** | 6,885 | chunk_sched/cmp456_chunkmono | SCHED (2 сайта: getChunkNow + moonrise$setFullChunk) | **LIVE** — МЕРЖ №10 в мастере; natives ровно 2 (schedProbe ()J, mirrorEvent (JZ)Z) |
| ChunkSendOps | 10,222 | chunk_send/cmp437_chunk4 | SEND | armed в юнионах chunkmono/c98ai (chunk_send.rs:124) — занято |
| ChunkPacketEncodeOps | 8,076 | chunk_send5/cmp444_chunk5 | SEND-packet | armed в тех же юнионах — занято |
| ChunkParseOps | 12,436 | chunk_parse/cmp420_chunk2 (∪cmp453_diet/cmp450_chunk) | LOAD-parse | блоб жив, плейн **REFUTED_CENS на банке** (Л144) |
| PoiOps | 7,356 | poi_plane/cmp456_poi | LOAD-poi/POI-тик | armed в poi-юнионах (poi456-4 окно жлёт — занято climb'ом) |
| NormalNoiseBatchOps (+Handle/Reaper/Recorder/TL) | 18,195 | noise_fill/CRUSSTY_NATIVE_NOISE_FILL, cmp452_mega | GEN-noise | dorm, capture=0 на банке (RC1) |
| PerlinNoiseNativeOps (+Handle/Reaper) | 6,566 | perlin_noise/CRUSSTY_NATIVE_PERLIN_NOISE **ON by default** (TASK-148) | GEN-synth | всегда ARMED и всё равно 0.0% — ген-ось инертна |
| ImprovedNoiseNativeOps (+Handle/Reaper/+batch) | (improved_noise.rs ×4) | CRUSSTY_NATIVE_IMPROVED_NOISE, **off by default** | GEN-synth | dorm |
| (PrepareOps/cmp401_offthread) | — | prepare_manager.rs — **мёртвый файл: `mod prepare_manager` ОТСУТСТВУЕТ в lib.rs (69 модов), prepare/ в git никогда не было** | SCHED-prepare | НЕ КОМПИЛИРУЕТСЯ — артефакт-призрак (найдено при картировании) |

## 3. Capture-матем по стадиям на БАНКЕ (закон 18: lane% × max-capture% < 20пп)

1. **GEN/noise: 0.0% soak-CPU** (RC1 TASK-421-C: worldgen/noise = 42 сэмпла = 0.0% BUCKETS_3 ch420b; §107: 0.0%/85 сэмплов). Kill-100% потолок = **0пп < 20пп** → **REFUTED_CENS** всего gen-класса ливеров на банке (aquifer-cell-cache, native-noise апгрейды, NoiseRouter-DAG). Мир MineShield-3 ПРЕГЕНЕРИРОВАН: forceload = PARSE сохранённых чанков, НЕ worldgen (RC2: parse-бурст 16384 миссов за ~2-3s @boot, 0 в окне поллов).
2. **LOAD/parse: 0.0155% incl** (Л144: SerializableChunkData 18/116,469 CPU, alloc 0/3,803; ВСЕ сэмплы на PrioritisedQueueExecutorThread, на тик-потоках 0) × kill-100% ≤ **0.02пп** = 25× ниже закона-13a → **REFUTED_CENS** (re-arm cmp420_chunk2 на банке бессмыслен; re-arm = «мир под давлением»).
3. **SAVE/write: write() = 69 инстр, boot/stop-only** (tick-438 «write-serialization placebo»; RC2 хронология) → потолок ≈ **0пп** → **REFUTED_CENS**; блоба нет, армить нечего.
4. **SEND/packet: capture≈0** (fp4/EmbeddedChannel, krypton-канон «сетевой слой бенч-невидим», Л-466-C95.2). Стадия занята (chunk4/chunk5 в юнионах), их +9.1..+17.1 ноги эры = GC-debt relief (RC6-механика), не send-lane capture.
5. **Единственная live-стадия пайплайна на банке = SCHED/tick-ось**: capture срезы 1.5-2.5% (Л26: due-ness + queue + Long2Ref 2.21%), NCH hot ~60/3,931 инстр = 1.5% поверхности — **занята** ChunkSchedOps (2/2 ретаргета live); свободный суб-сайт (P22-маска, не имплементирована — Л103) = net-new +0.87-1.55пп < 20пп (суб-бар, climb-компонент, НЕ свободная стадия verdict-класса).

## 4. Топ-свободная стадия (вне REFUTED-стены ×11 и вне занятых)

**GEN/noise** (computeSubstance 278 инстр + noise(DDDDD) 106 инстр + fillSlice 91): единственная стадия, где (i) нет собственного REFUTED-входа механики (REFUTED «FFI-миграция ≤+1.2» = маршрут доставки, не стадия; «zero-copy/striped/affinity» — не gen), (ii) внешний перенос-сигнал +3-7% c/s (C2ME #558/#557) — самый крупный в 19a, (iii) capture на банке = 0 → верикт-веню НЕ банк, а 19a-стратум живой генерации (стресс-ladder: chunky-pregen миры с edge-gen, radius-стратум 480 = 56.25% плоскости = (480/640)², r640 канон, r1280 FAILURE-диагноз S07).
Проверяемый inputs-рычаг стратума ЛЕГАЛЕН: radius — INPUT world-bench-parallel.yml; r480-нога квантифицирует плоскость→TPS transfer на живом CI.

## 5. Гипотеза-дельта (prereg, закон 16) → диспатч round-468-s62-*

- **s62-a, s62-b (0 код-дельт, бан-EXACT r640 ваниль)**: (а) каждая in-band нога = в-точка банка §3 + сэмпл вопроса базы МЕРЖ №10 (две глубокие ванили −13.32@6.92M / −13.61@6.97M vs флор-шум z=−1.83σ — canary-407/408 не абсорбированы, мои точки = независимые арбитры); (б) попадание в E-окно chkclimb-5 [6427199,6527199] с norm ≤−1.60 + Δcpu ≤50k к легу = якорь МЕРЖ №11 (окно «жлёт якорей»).
- **s62-c (0 код-дельт, WILD-стратум: radius=480, остальное бан-EXACT)**: prereg — (а) GEN-lane остаётся инертной на r480 (RC1 канон: forceload=parse; предсказание 0 gen-маркеров в окне), (б) наблюдаемое Δ = нелинейность norm_v5 (калибровка r640): прогноз skew **+4.8..+6.8пп** (WILD-v6 x464-абсорб), (в) численная пара (medTPS_r480, medTPS_r640) от моих же 0-дельт ног s62-a/b = первая pair-controlled квантификация 19a-стратума: Δmed = эластичность плоскости чанков→TPS для gen-стадии. NOT merge-pair (WILD).
- Если s62-c покажет gen-lane > 0% в окне (против RC1) — гипотеза-дельта для aquifer-cell-cache получает в-точку capture и venue-пересмотр; иначе gen-стадия закрывается REFUTED_CENS и на r480-стратуме (банк-семейство), venue = только стресс-v2.

## 6. Диспатч (факт) — см. S62.md ФИНАЛ
