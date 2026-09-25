# RESEARCH-459-M2 — RUST bulk-JNI дельта-сериализация чанков (revisit ID-M2, закон-11 WILD)

TASK-459-79 | тик-459 (закон 11, v18.2/18.3) | ветка round-459-m2 (base origin/round-458m-delta @d13eb80a) | вектор: send/serialize lane чанков
Статус: STEP-1 SCAFFOLD DORMANT (rust-модуль wired в lib.rs, java-стаб в репо, редирект НЕ ставится — гейты G1-G6 до ARM)

## 1. ПРОФИЛЬНАЯ БАЗА (числа, сэмплы, НЕ дублируем чужие оси)
Из карточки agent-M (RESEARCH-458-M.md @d13eb80a, агрегация round-chkmono457-11: cpu-collapsed 104163 / alloc-collapsed 3029 сэмплов):
- send/serialize lane (PlayerChunkSender|ClientboundLevelChunk|ChunkSendOps|LevelChunkPacketData): **1.75% CPU** — ОСЬ M2
- chunkmap-send total (ChunkMap.tick+ChunkHolder+LightEngine): 6.01%; paletted read lane 5.58% — ЧУЖАЯ плоскость (cmp457_paldelta, не дублируем)
- GC total ~3.9% CPU (PSCardTable::scavenge 1.56% + oop-iterate 1.38%) — HIT-путь убивает encode-аллокации (побочный эффект, в вердикт-число НЕ входит)
- serialize/save (диск): 0.00%; worldgen/noise: 0.15% — оси инертны на фиксстуре

## 2. JAVAP GROUND TRUTH (МОЙ прогон, 2026-09-25, round-396-a patched-kernel.jar)
1. 2-arg `ClientboundLevelChunkPacketData.extractChunkData(FriendlyByteBuf, LevelChunk)` — байткод: `aload_0; aload_1; aconst_null; invokestatic 3-arg; return` = ТРИВИАЛЬНЫЙ делегат → идеальный static→static redirect, sites:1, дескриптор `(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/world/level/chunk/LevelChunk;)V`
2. 3-arg тело: `iconst_0; istore_3; aload_1; invokevirtual LevelChunk.getSections()` → цикл `section.write(buf, packetInfo, i++)` — ЧИСТАЯ функция секций, zero cross-section state → пер-секционная декомпозиция ТОЧНА
3. `LevelChunkSection.write(FriendlyByteBuf, ChunkPacketInfo<BlockState>, int)` — public (javap -p)
4. `LevelChunk.isUnsaved()Z` — public, chunk4-сертифицированный dirty-оракул (change→mark ДО следующего send-probe)
5. Апстрим-подтверждение: PaperMC paper-server patch 0025-Anti-Xray сам вводит 2-arg как делегат `extractChunkData(buffer, chunk, null)` (см. URL-1, строки 103-107 патча) — dual-path дизайн не наша выдумка
6. Оракул ИЗМЕРЕН: `ChunkDeltaM2Ops.selfTest()Z` = **TRUE** против round-396-a kernel jar на sandbox JVM (9/9 shape-шагов OK; authlib6 link-стабы ТОЛЬКО в probe-класспатче /tmp, не в репо); fail-closed продемонстрирован — на неполном класспаче оракул вернул false (Throwable→false, ARM невозможен)

## 3. CAPTURE-МАТЕМ (Δ числом; preregistered)
- **Δ = lane × capture = 1.75пп × 0.70 = 1.225 → Δ +1.2пп; потолок = 1.75пп** (lane × 100%)
- basis capture 0.70: bench-4 (4 фейк-игрока, view-distance overlap) = до 4 constructions одной ревизии чанка; chunk4-cap 2048 против ~20k чанков = постоянный turnover → constructions ПОСЛЕ эвикшена и dirty-реконструкции ловит пер-секционный HIT (та же ревизия секции → ваниль пишет идентичные байты)
- GC-хвост: HIT-путь аллоцирует только writeBytes-копию (encode-аллокации paletted Strategy/varint исчезают) — до +0.1пп от 3.9% GC, отдельной строкой, в Δ НЕ суммируется (честно)
- формула запина в коде: `chunk_delta_m2::capture_math(1.75, 0.70)` + cargo-test (число вычисляется, не выдумывается)

## 4. ПОЧЕМУ PARK ×458 РАСТВОРИЛСЯ (дельта против карточки M)
- PARK-причина (a) «palette-порядок java-HashMap → rust re-encode не bit-in-byte»: M2 **НЕ перекодирует** — единственный писатель байт = ванильный 3-arg цикл; rust хранит/реплеит ТОЛЬКО ванильно-записанные байты (readback capture) → закон-4 паритет по построению
- PARK-причина (b) «JNI-маршалинг МБ/тик»: bulk-JNI = ОДИН crossing/тик, в партии ТОЛЬКО изменённые секции (дельта), не полные payload; неизменные секции реплеятся из rust-кэша без crossing
- что осталось честным риском: hit-rate на фиксстуре (lane 1.75% — маленькая ось); вердикт едет на carrier-лесенке (канон chunk5), не standalone

## 5. SCAFFOLD ЭТОГО КОММИТА (12e: wiring + write-through, не «рекомендую»)
| Файл | Что | Канон |
|---|---|---|
| src/chunk_delta_m2.rs | STRICT-gate cmp459_m2chdelta, consts дескрипторов (javap), pristine-hook (всегда None = ваниль), activate no-op + step-2 порядок, capture_math + 2 теста | chunk_send5 STRICT-gate, law 6/7 |
| chunksend/net/minecraft/world/level/chunk/ChunkDeltaM2Ops.java | flat, zero nested/lambda, major 65: selfTest()Z структурный оракул (4 pinned shape), extractChunkData 2-arg = ваниль passthrough (будущий redirect-target), leverId() | NCDFE-канон define-before-first-touch, dormancy |
| src/lib.rs | mod + register() + activate() wiring | паттерн chunk_send5 |
 dormant-семантика: пустой/чужой lever_flag = ZERO хуков, ваниль бит-в-бит; flag=cmp459_m2chdelta = pristine-sighting лог, редирект всё равно не ставится до step-2 (selfTest до ARM, урок-408)

## 6. PREREGISTERED ГЕЙТЫ G1-G6 (step-2, до диспатча на world-bench)
- G1 ARM-маркер `[crussty-plugin] cmp459_m2chdelta: ARMED` только после define+selfTest==true (урок-408 ×2: paldelta-1, roar-2)
- G2 lockstep бит-в-байт оракул: первые 2 капчура → второй ванильный encode в scratch → посекционное сравнение; FAIL → DISABLED latch → ваниль навсегда
- G3 NCDFE=0; threw=0; AIOOBE=0; pop 140-165k паритет; items 0.00
- G4 band 6.0-9.5M, min-of-3, pair Δ≤50k pair-fresh (занят = в МЕРЖЕ)
- G5 TPS-бар vs банк v4 (chk-14 +21.7@8687055 и др.); Δ-прогноз +1.2пп, потолок 1.75пп
- G6 fail-closed: любой Throwable → disarm, vanilla body; zero added JNI/тик кроме одного bulk crossing

## 7. STEP-2 ПЛАН (не в этом коммите)
build-script chunksend (flat==nested x93) → classfile.rs consts+resolution closure+pristine guard → redirect install (sites:1) → rust bulk-JNI batch (jni_table, один crossing/тик) → delivery tests по chunk_send-канону → carrier-union c cmp457_paldelta-сайтами (синхронно java+rust, урок ×451/×452)

## 8. SOURCES (интернет, curl 2026-09-25, оба 200 OK)
- URL-1: https://github.com/PaperMC/Paper/blob/main/paper-server/patches/features/0025-Anti-Xray.patch — апстрим-доказательство тривиального 2-arg делегата (строки 103-107: `extractChunkData(buffer, chunk, null)`) и пер-секционного writer-контракта (строка 336)
- URL-2: https://github.com/CaffeineMC/lithium-fabric — paletted/codec machinery, которую M2 skip-ает на HIT (encode-плоскость = та же, что lithium оптимизирует; наш выигрыш — НЕ вызывать её вообще на повторных construction)
- Локальные каноны: RESEARCH-458-M.md (@d13eb80a) — javap-карточка agent-M; chunk_send5.rs/chunk_send.rs — STRICT-gate + selfTest-до-ARM; BOTTLENECK.md ×459 — гейты и банки
