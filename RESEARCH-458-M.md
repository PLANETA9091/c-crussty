# RESEARCH-458-M — дельта-сериализация чанков (chunk/worldgen ось, закон-11 БЕЗУМИЕ)
TASK-458-M | тик-458 (22:08 +08 2026-09-25) | ветка round-458m-delta @96cc2704 | вектор: chunk parse/serialization/send pipeline

## 1. ПРОФИЛЬНАЯ БАЗА (evidence-first, NOT-A-BENCH гвард)
Агрегация round-chkmono457-11 (серт-носитель, cpu-collapsed 104163 / alloc-collapsed 3029 samples, из git@96cc2704):
- serialize/save lane (ChunkSerializer/RegionFileStorage/NbtIo): **0.00% CPU** — диск-сохранение на преген-фикстуре НЕВИДИМО
- send/serialize lane (PlayerChunkSender|ClientboundLevelChunk|ChunkSendOps|LevelChunkPacketData): **1.75% CPU**; chunkmap-send total (ChunkMap.tick+ChunkHolder+LightEngine): **6.01%**
- paletted read lane 5.58% (ЦЕЛЬ cmp457_paldelta — НЕ дублируется, другой сайт: PalettedContainer.get — READ-плоскость игры, не send)
- packet construction ALLOC: **0.00%** — construction-аллокации невидимы в steady-state; entity-allocs (Vec3 13.9% + AABB 12.8%) — ЧУЖАЯ ось (agent-I ID-H01)
- GC total ~3.9% CPU (PSCardTable::scavenge 1.56% + oop-iterate 1.38%) — «GC-debt relief +6-8пп» механизм оси
- worldgen/noise: 0.15% CPU — ×421-C подтверждена (GEN-ось инертна на преген-фикстуре)
ВЫВОД: lane маленькая → честный захват ≤1-2пп CPU + GC-хвост; вердикт ноги едет на CERT-CARRIER (прецедент chunk5: «monotone, parity-transparent member of the law-8 axis; the legs' verdict rides the cmp437_chunk4 carrier it widens»).

## 2. JAVAP GROUND TRUTH (pinned kernel round-396-a, 2026-09-25)
- `ClientboundLevelChunkPacketData.extractChunkData(FriendlyByteBuf, LevelChunk)` 2-arg = ТРИВИАЛЬНЫЙ делегат → 3-arg(null) — идеальный static→static redirect (chunk4-механика, sites:1)
- 3-arg тело (javap-c): `int i=0; for (section : chunk.getSections()) section.write(buf, packetInfo, i++); if (buf.writerIndex() != buf.capacity()) throw new IllegalStateException(fmt(capacity, writerIndex))` — ЧИСТАЯ функция секций, zero cross-section state
- `LevelChunkSection.write(FriendlyByteBuf, ChunkPacketInfo, int)` = `buf.writeShort(nonEmptyBlockCount); states.write(buf, packetInfo, i); biomes.write(buf, null, i)` — public, tiny
- anti-xray: shouldModify → ctor(LevelChunk, ChunkPacketInfo) → 3-arg НАПРЯМУЮ (2-arg невидим anti-xray — плоскость безопасна)
- isUnsaved() в kernel = композит (getGameTime + blockTicks moonrise-вызов + unsaved field) — СЕРТИФИЦИРОВАННЫЙ dirty-оракул chunk4 («any block/block-entity change marks the chunk BEFORE the next send probes»)
- ctor прекалибрует буфер: writerIndex()==capacity() инвариант в конце vanilla extract → replay идентичного total байт сохраняет инвариант

## 3. §БЕЗУМИЕ — 3 безумные идеи (закон 11)

### [ID-M1] ПЕР-СЕКЦИОННАЯ ДЕЛЬТА-ПЛОСКОСТОСТЬ extractChunkData — **GO**
**Суть**: вместо полной пере-сериализации секций на каждый construction — кэш пер-секционных payload byte[] (записанных САМОЙ ВАНИЛЬЮ, readback-капчур) + replay на HIT. Dirty-модель = СЕРТИФИЦИРОВАННЫЙ isUnsaved-оракул chunk4 (строго туже chunk4: probe-at-serve + owner-identity WeakRef). Композируется ВЫШЕ chunk4/chunk5: chunk4-HIT пропускает extract целиком; мой HIT ловит constructions ПОСЛЕ chunk4-эвикшена (cap 2048 vs ~20k чанков = постоянный turnover) и dirty-реконструкции.
- Почему ≥+20: вердикт едет на carrier-лесенке (cert-stack ⊕ плоскость); в окнах joins-burst lane концентрируется (RESEARCH-F: write path = то же paletted/codec machinery в burst-окне); GC-debt: HIT-путь аллоцирует только writeBytes-копию (encode-аллокации paletted Strategy/varint исчезают) — механизм оси +6-8пп при повторных construction-волнах.
- Риск: hit-rate на фикстуре скромный (lane 1.75%); СВОБОДНЫЙ residual = сертифицированный класс isUnsaved-оракула (change→save→probe окно — ТОТ ЖЕ, что у chunk4; мой serve-гейт = owner-identity ∧ !isUnsaved() = строго не слабее).
- Parity (закон 4): HIT-байты = байты, ЗАПИСАННЫЕ ванильным encode (readback-капчур) — bit-in-byte по построению; online selftest (первые 2 капчура: ВТОРОЙ ванильный encode в scratch, посекционное сравнение бит-в-бит; FAIL → DISABLED latch → ваниль навсегда); fail-closed на любой Throwable; пустой флаг = класс не определяется, тело бит-в-байт ваниль.
- GO: law 6 — 0 добавленных JNI/тик (selfTest однократный в boot); подсистема целиком: bridge + redirect + guards + tests + carrier.

### [ID-M2] RUST bulk-JNI дельта-движок (Rust владеет пер-секционными буферами, ОДИН bulk-JNI/тик отдаёт изменённые палитры/массивы) — **PARK**
Почему не в этот тик: (a) palette-порядок в серилизованном буфере определяется итерацией java-HashMap палитры контейнера — rust re-encode не даёт bit-in-byte без захвата порядка (риск закона 4 непроходим); (b) JNI-маршалинг МБ/тик (4-байтный буфер × секций × чанков) съедает больше, чем освобождает на lane 1.75%; (c) Java-capture (ID-M1) даёт byte-exact бесплатно (байты пишет сама ваниль). Revisit: если ось станет стеной (>5% lane) и palette-порядок стабилизируется (sorted-транзакции) — тогда rust-кодек поверх java-эпох.
- Риск: закон 4 (палитра-порядок), JNI-стоимость, один bulk-JNI/тик на MB-объёмах.
- Parity-план: невозможен без java-привязки → ID-M1 субсумирует.
- PARK.

### [ID-M3] PRE-ENCODED протокол-пакеты на promote-фазе (сдвиг сериализации в worldgen-burst) — **PARK (REFUTED by canon)**
Сдвиг стоимости между окнами = NOT-A-BENCH/местебо по канону (тот же total CPU); ChunkMap.save уже скипает !isUnsaved (ваниль идентична) — нулевой честный Δ. Альтернатива-кандидат (light-nibble кэш по DataLayer identity) отклонена: DataLayer мутирует IN-PLACE без версии — stale-риск непроходим закона 4.
- PARK (честно рефьютнуто профилем + каноном).

## 4. ПЛАН ИМПЛЕМЕНТАЦИИ (law 6: подсистема целиком)
1. Carrier-union: cmp458_chdelta во ВСЕ gate-сайты, несущие cmp457_paldelta (rust 21 файл + java 10 blob-источников, синхронно — mirror-drift урок ×451/×452), paletted.rs ИСКЛЮЧЁН (paldelta-плоскость не едет на моём carrier — другая нога, не дублируем)
2. Bridge: chunksend/net/minecraft/world/level/chunk/ChunkDeltaOps.java (flat, zero nested/lambda, major 65, netty — рефлективная поверхность по chunk5-канону)
3. Redirect: static body extractChunkData 2-arg → ChunkDeltaOps.extractChunkData (exact descriptor, sites:1, anti-placebo)
4. Rust: src/chunk_delta.rs (STRICT cmp458_chdelta gate, EARLY-define canon 5ecd841a/d73758a3: define gate несёт флаг, define STRICTLY before first touch, selfTest==true ДО ARM) + classfile.rs (consts + resolution closure + pristine guard) + lib.rs wiring
5. Tests: delivery tests по chunk_send-канону (kernel fixture ClientboundLevelChunkPacketData.class из round-396-a jar, sha256-identical; redirect sites:1; idempotency; flat; major 65; blob cp carries cmp458_chdelta)
6. cargo check --lib зелёный → dispatch world-bench-parallel leg-1 (inputs по мандату, lever_flag=cmp458_chdelta)

## 5. SOURCES (интернет-рисёрч, curl 2026-09-25)
- GitHub API repo/commit state PLANETA9091/c-crussty master (200 OK, pushed 2026-09-25T14:45:47Z)
- PaperMC/Paper code search: extractChunkData → paper-server/patches/features/0025-Anti-Xray.patch (анти-ксай 3-arg перегрузка — подтверждение dual-path дизайна из javap)
- PaperMC/Paper README (paper-server layout; patches/ модель — "Vanilla patched" дисциплина mirrors our redirect-канон)
- minecraft.wiki/Protocol — 403 bot-gate (обход: локальный javap kernel = высшая инстанция ground truth)
- Локальные каноны: chunk_send.rs/chunk_send5.rs (certified precedents), scripts/build_438c_chunksend.sh (build-дисциплина, x93 flat==nested), scripts/add_paldelta_gates_457.py (carrier-union механика), 5ecd841a (NCDFE arm-AFTER-define)
