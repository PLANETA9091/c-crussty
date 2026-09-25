# RESEARCH-459-L07 — chunk-send/serialization: аудит round-458m-delta + capture-матем send-лейна (v18.3, законы 13-16)

TASK-459-L07 | тик-459 | слот L07 (агент-M ветка round-458m-delta, ID-M1) | профили: round-chkmono457-14 (монстр-нога), 457-11 (депресс), 457-16, anchor458-33

## 1. GIT-АУДИТ ВЕТКИ round-458m-delta
- Ветка @d13eb80a = ровно 1 коммит поверх 96cc2704: «TASK-458-M step-1: RESEARCH-458-M.md» — +54 строки документа, **0 строк кода**. Ветка RESEARCH-ONLY.
- Поиск по всему репо: `ChunkDeltaOps|CRUSSTY_CHUNK_DELTA|src/chunk_delta.rs|extractChunkData-redirect` = **0 хитов в коде**; упоминания только в docs/LAB_LEDGER.md:40 и docs/GOAL_20TPS_MINESSHIELD3.md:1983.
- Статус по BOTTLENECK ×459: «swar/delta/noisesimd — карриеры в очереди» — подтверждено: карриер cmp458_chdelta НЕ реализован.
- Шаги 1-6 плана имплементации RESEARCH-458-M (bridge → redirect → rust → carrier-union → тесты → cargo+dispatch): **все NOT STARTED**.

## 2. БЛОБЫ: javap/strings ГЕЙТ СПЯЩИХ ГЕЙТОВ ×425
- `chunksend/build/net/minecraft/server/network/ChunkSendOps.class` — ЖИВОЙ (chunk4-сертификат в master): strings = CARRIER_UNION_435/437/444/450/452/453, «cmp437_chunk4: chunk4 snapshot selftest PASS/FAIL», stats sent=/serialized=/hits=/evictions=. Форма: CACHE = ConcurrentHashMap<Long, ClientboundLevelChunkWithLightPacket> (целый пакет, cap 2048), счётчики sent/serialized/hits/evictions, selftestLeft.
- `paletted/build/net/minecraft/world/level/chunk/PalettedContainerOps.class` — ЖИВОЙ: crusstyMiss/crusstyEpoch/crusstySnapGen/crusstySnap/selfTest, «paletted: builds=%d aborts=%d capped=%d live=%d».
- **cmp458_chdelta strings в блобах: 0 из всех** — левер НЕ вшит ни в один блоб (плацебо-риск отсутствует лишь потому, что кода нет вовсе). При будущем вайринге ОБЯЗАТЕЛЕН гейт: javap flat==nested 10/10 EQUAL + strings cmp458_chdelta ≥1 хит в каждом блобе cp (прецеденты: ×425, ×458-F1 agent-N 0/10, roar-2 DELIVERY-FAIL ×459).
- Композируемость: chunk4-HIT пропускает extract целиком; ID-M1 ловит constructions ПОСЛЕ эвикшена cap-2048 (~20k чанков = постоянный turnover) и dirty-реконструкции — пер-секционный слой НИЖЕ целого пакета, конфликтов нет.

## 3. JAVAP-КОНТРАКТЫ (4, kernel round-396-a sha-pinned + блобы)
1. `ClientboundLevelChunkPacketData` (kernel): **2-arg static extractChunkData(FriendlyByteBuf, LevelChunk)** = redirect-сайт sites:1; рядом 3-arg overload с `io.papermc.paper.antixray.ChunkPacketInfo` (anti-xray dual path); поля heightmaps/buffer(2MB cap TWO_MEGABYTES)/blockEntitiesData/extraPackets.
2. Тело 3-arg (javap -c, точная транскрипция): `i=0; for (s : chunk.getSections()) s.write(buf, packetInfo, i++); if (buf.writerIndex() != buf.capacity()) throw new IllegalStateException(makeConcat(capacity, writerIndex))` — ЧИСТАЯ функция секций, zero cross-section state → пер-секционный кэш+replay структурно sound; инвариант writerIndex==capacity сохраняется при replay идентичного total байт.
3. `LevelChunkSection.write(FriendlyByteBuf, ChunkPacketInfo, int)`: `writeShort(nonEmptyBlockCount); states.write(buf, info, i); biomes.write(buf, null, i)` — public, самодостаточный пер-секционный payload → readback-капчур ванильного encode = bit-in-byte по построению (закон 4).
4. `ChunkSendOps.sendChunk(ServerGamePacketListenerImpl, ServerLevel, LevelChunk)` static — та же сигнатура, что `PlayerChunkSender.sendChunk` static (kernel) = точка хука chunk4; `PlayerChunkSender.sendNextChunks(ServerPlayer)` c batchQuota/MIN/MAX_CHUNKS_PER_TICK/MAX_UNACKNOWLEDGED_BATCHES = поверхность P26 coalescing.

## 4. ПРОФИЛИ + CAPTURE-МАТЕМ (монстр-нога chkmono457-14)
Декомпозиция wall-collapsed chkmono457-14 (вес-суммы, 63661 сэмплов, окно 55-80% soak):
- **chunk-send serialize лейн** (PlayerChunkSender | ClientboundLevelChunk | LevelChunkPacketData | ChunkSendOps | ChunkSerializer | sendChunk): **0 сэмплов = 0.00%** — подтверждение и расширение LEDGER-числа агента-M (0.00% на soak) на монстр-ногу.
- **entity-sync send лейн (wall)**: SynchedEntityData.* 239 (0.375%) + ServerEntity.sendChanges 82 (0.129%) + ChunkMap$TrackedEntity.* 101 (0.159%) + Entity.moonrise$getTrackedEntity 12 = **433 сэмпла = 0.68%**.
- CPU-окно (BOTTLENECKS_3, 103062 сэмплов): network bucket 3368 = 3.3%; phase network sync (ServerEntity) 2696 = 2.6%; sendChanges 1127 = 1.1%; SynchedEntityData.getValue 1840 = 1.8% + getItem 1225 = 1.2%; TrackedEntity.moonrise$tick 819 = 0.8%; ChunkEntitySlices.getEntities 1120 = 1.1%. ABSORB 457-14: players_packets 0.01% (флэт), lane items 31.17→0.00, inside_volatile 12.01→15.36 РОСТ.
- Range по 3 ногам CPU-окна (все fake_players=4, статичные): network 2.7-3.3%, net-sync phase 1.9-2.6%, sendChanges 0.6-1.1% (457-16 @6951662: 913=0.9%; anchor458-33 @6973621: 718=0.6%).
- GC/alloc на 457-14: F2 alloc-churn ≈0 MB/s (вес 3896 байт), Full=9/young=128 — GC-debt relief чдельты на этой фикстуре = 0.

CAPTURE-МАТЕМ (lane% × захват% = Δ; потолок = lane% × 100%):
| лейн | lane | захват | Δ-прогноз | потолок |
|---|---|---|---|---|
| ID-M1 serialize, монстр-нога 457-14 | 0.00% | 85% | **+0.0пп** | **+0.0пп** |
| ID-M1 serialize, joins-burst/серт-фикстура (замер 457-11) | 1.75% | 85% (bit-exact replay) | **+1.5пп** | **+1.75пп** |
| P26/P27 entity-sync coalescing (смежный лейн, CPU-окно) | 2.6% | 35% | **+0.9пп** | **+2.6пп** |

## 5. ЗАКОН-8 ВИДИМОСТЬ
chunk-send = игрокам-ВИДИМЫЙ лейн (загрузка чанков на клиенте). Но на текущих soak/монстр-фикстурах fake_players статичны у спавна → лейн пуст (0.00% wall, 0.01% players_packets) → Δ чдельты там непроявим в принципе. Проявление лейна: joins/movement-burst окна (прецедент chunk4-серт: verdict rides carrier) и **world-bench с реальным движением игроков**. Вывод: диспатч cmp458_chdelta на монстр-ногу = гарантированный ноль; целевой рантайм — world-bench-parallel.

## 6. PREREGISTERED ГЕЙТЫ G1-G6
- G1 ARM+эффект-маркеры: selftest PASS в логе + hits>0 в «chunk-delta stats» до пурджа; нет маркера = DELIVERY-FAIL (урок-408 ×2).
- G2 lockstep бит-в-байт оракул: первые 2 капчура — второй ванильный encode в scratch, посекционное сравнение бит-в-бит; FAIL → DISABLED latch → ваниль навсегда.
- G3 young/Full GC: Full ≤9, young в банке-справке; GC-churn relief = маркер, не Δ.
- G4 популяция-паритет: 140-165k living, pop VALID, spawnable-chunks >0.
- G5 TPS-бар vs банк v4: pair-fresh якорь, Δ≤50k, min-of-3, band [6.0M, 9.5M], norm = median/TPS_exp − 1.
- G6 fail-closed disarm: любой Throwable → ваниль; AIOOBE=0; NCDFE=0 ДО вердикта (EARLY-define в раннем arm-хуке, прецеденты d73758a3/5ecd841a/9d71b461).

## 7. ПРОГНОЗ + ПЛАН ДО МИР-БЕНЧА (что осталось)
- Прогноз ноги на монстр-фикстуре: **+0.0пп — НЕ диспатчить** (lane 0.00%).
- Прогноз на joins-burst: +1.5пп CPU прямой; сам по себе суб-бар → ехать компонентой cert-stack (прецедент chunk4/chunk5: «verdict rides the carrier it widens»), плюс GC-debt carrier в burst-волнах (на 457-14 механизм = 0).
- Остаток работ (шаги RESEARCH-458-M, все NOT STARTED, оценка ~1 имплементор-тик):
  1. Bridge `chunksend/net/minecraft/.../ChunkDeltaOps.java` — flat, zero nested/lambda, major 65.
  2. Redirect: static body extractChunkData 2-arg → ChunkDeltaOps (exact descriptor, sites:1, anti-placebo).
  3. Rust `src/chunk_delta.rs` — STRICT cmp458_chdelta gate, EARLY-define, selfTest==true ДО ARM + classfile.rs consts + lib.rs wiring.
  4. Carrier-union cmp458_chdelta во ВСЕ gate-сайты cmp457_paldelta (rust 21 файл + java 10 blob-источников, mirror-sync).
  5. Ребилд блобов + javap flat==nested 10/10 EQUAL + strings cmp458_chdelta ≥1/блоб (×425-гейт) + cargo test зелёный.
  6. Dispatch world-bench-parallel leg-1 (lever_flag=cmp458_chdelta, inputs по мандату) — НЕ монстр-soak.

## 8. ИСТОЧНИКИ (≥3)
1. **PaperMC/Paper issue #5920** «Anti-X-Ray conflict with custom world generation World Height» — fetched GitHub API 2026-09-25: подтверждает dual-path дизайн 3-arg ChunkPacketInfo overload (мой javap контракт #1).
2. **Paper patches 0025-Anti-Xray.patch** (fetch агента-M ×458, подтверждено моим javap kernel): anti-xray пишет через 3-arg НАПРЯМУЮ, 2-arg невидим anti-xray → redirect-плоскость безопасна.
3. **CaffeineMC krypton** (flush-consolidation/netty batching) — механика P26 send-burst coalescing (+0.3-0.8пп по LEDGER); GitHub API rate-limited в тик, источник канонизирован ранее.
4. **Moonrise (Spottedleaf)** chunk-system: send-path/трекинг фреймы в профилях (moonrise patches 5.5% CPU на 457-14; ChunkMap$TrackedEntity.moonrise$tick 0.8%; moonrise$getTrackedEntity) — поверхность P26/P27.
5. Локальные каноны: chunksend/build ChunkSendOps (certified chunk4), scripts/build_438c_chunksend.sh (build-дисциплина flat==nested), round-396-a kernel jar (sha-pinned javap ground truth).
