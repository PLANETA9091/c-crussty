# RESEARCH-450-C — TASK-450-C (chunk-pipeline axis, закон 8, carrier cmp450_chunk)

Agent: TASK-450-C (tick-450, 20:0x +08, Job 406609). Base: fresh master d02a2977
(код = merge-base d282985c — нулевой код-дрейф, verified `git log d282985c..origin/master -- src/` =
пусто). Носители: chunk4 = round-438-c-chunk4b @c5fe0251 (cmp437_chunk4), chunk5 =
round-444-b-chunk5 @2a07c491 (cmp444_chunk5, ПОСТРОЕН ПОВЕРХ chunk4 — merge-base(c4,c5)=c5fe0251).

## 1. Мандат и миссия

≥+20% pair-stable ОБЯЗАТЕЛЬНО (закон 3). Chunk-pipeline = parse/serialization/send
ЦЕЛИКОМ (закон 6 масштаб подсистемы). Слесы-кандидаты из промпта: RegionFile
IO-buffering, chunk serialization batch, packet encode — «по research» (ниже: честные
ценз-вердикты по каждому). STRICT-OR флаг-гейт, ваниль при пустом флаге бит-в-байт.

## 2. Что уже несёт юнион (после merge 634195f9)

| плоскость | носитель | механизм |
|---|---|---|
| chunk_parse (section-codec caches) | cmp420_chunk2→cmp435 | block_states deep cache (cap 16384) + biomes-parse cache — оба section-лямбды (RESEARCH-F: parse burst 33.38% = codec 19.06 + paletted 13.98) |
| chunk_send (send snapshot) | cmp437_chunk4 | PlayerChunkSender.sendChunk body-redirect, revision/unsaved-keyed packet reuse (cap 2048 evict-half): 4× дубль сериализации на чанк → 1× (RC6 GC-debt relief в join-бурсте) |
| chunk_send5 (encode cache) | cmp444_chunk5 | ChunkPacketEncodeOps.write instance-keyed payload cache: повторный packet.write() на того же игрока → byte[] replay |

## 3. javap-ценз чанк-pipeline классов (kernel round-396-a, paper 1.21.10 mojang-mapped)

- `RegionFileStorage`: `read(ChunkPos)->CompoundTag`, `write(ChunkPos,CompoundTag)`,
  `scanChunk`, `moonrise$readData/moonrise$finishRead` (async IO-контроллер),
  `regionCache` Long2ObjectLinkedOpenHashMap, `sync` flag. Чтение = header seek +
  `RegionFile.getChunkDataInputStream` (inflate) + NBT decode — ВСЁ на
  moonrise IO-воркерах (`PrioritisedQueueExecutorThread`), НЕ на main.
- `SerializableChunkData` (1.21.10 ChunkSerializer): record с `read(...) -> ProtoChunk`
  (parse saved chunk), `copyOf(ServerLevel, ChunkAccess)` + `write() -> CompoundTag`
  (serialization write-сторона), sectionData/entities/blockEntities lists.
- `GenerationChunkHolder`/`ChunkHolder`: scheduleChunkGenerationTask, save-dependencies —
  chunk-system scheduling (moonrise NewChunkHolder), не main-tick.
- `PlayerChunkSender`: `sendChunk(SGPL,ServerLevel,LevelChunk)` = ЕДИНСТВЕННЫЙ сайт
  посылки (уже перекрыт chunk4), `sendNextChunks` = батч-коллектор ванили,
  MIN/MAX_CHUNKS_PER_TICK rate-limit (пакет-пейсинг — игроки-видимый, НЕ трогаем).
- `MoonriseRegionFileIO`: scheduleSave/loadChunkData/loadAllChunkData — весь IO
 Prioritised/async; flushRegionStorages — autosave/stop.

## 4. Лан-профили (артефакты golden ×443/×449, ванильные якоря, стены)

- CPU self-time buckets (anchor-1 golden ×443): chunk system (kernel) **8.2%**,
  network **2.9%**; SimpleBitStorage.get 1.5% (parse codec leaf); на chunk4-1 ноге:
  chunk system 8.7% / network 2.2% (send snapshot уже снял дубль в cpu-окне).
- wall-окно (55-80%): RegionFile/ChunkSerializer/ChunkHolder ≈ 0-40 сэмплов / 61255 —
  чанк-работа в soak-стене отсутствует (бурст = boot/forceload/join до окна).
- alloc-окно (80-100%, round-round-443g-chk3-1): RegionFile-фреймы **311/4463 ≈ 7%**
  окна — ЦЕЛИКОМ на PrioritisedQueueExecutorThread (IO-воркеры, off-main);
  unload/GC-чанк-письма в soak.
- boot-Done ось (×424 chunksend): vanilla 15.2-16.9s → ноги 12.3-16.0s (−10..−27% на
  живых ногах) — игроки-видимый эффект.
- wgen-ось: закрыта ×445 (фикстура прегенерирована, fillFromNoise 0.0%, PerlinNoise
  bridge ON-BY-DEFAULT bit-exact) — НЕ ре-открывать.

## 5. Вердикты по срезам-кандидатам (анти-плацебо дисциплина RESEARCH-G)

| срез | вердикт | доказательство |
|---|---|---|
| RegionFile IO-buffering | **REFUTED для soak-фикстуры** | (a) лейн off-main (IO-воркеры) — TPS ест только через GC-debt; (b) кэш прочитанных NBT = 0 попаданий в бурсте (каждый чанк читается ОДИН раз; 9216 forceload) и parity-опасен при инвалидации (письмо→чтение мусора нельзя); (c) buffer-pooling внутри inflate = реимплементация zlib — вне бюджета/риска |
| chunk serialization batch (write) | **PLACEBO** | write-serialization в soak = 1 autosave в хвосте (RESEARCH-A таблица, канон ×438); RegionFile-аллокации soak = unload-writes off-main |
| packet encode | **УЖЕ В ЮНИОНЕ** | chunk5 encode-cache (cmp444_chunk5) покрывает re-encode; chunk4 snapshot покрывает 4× дубль первой сборки |
| snapshot cap 2048 → больше | **REFUTED анализом** | рабочий сет send-бурста = 4 игрока × 2-3 чанка/тик × спред ~20-50 тиков ≈ 400-600 чанков ≪ 2048 → mid-burst evictions нет; cap 16384 = +~1-1.5GB heap за ноль попаданий — GC-регресс |
| SerializableChunkData.read в Rust | **ВНЕ БЮДЖЕТА ТИКА** | полный parse NBT→ProtoChunk (registries, block entities, entities, heightmaps, ticks) — подсистема ЦЕЛИКОМ на следующий тик с отдельным research; частичный перенос = per-function вектор (закон 6 ЗАПРЕЩАЕТ как основной) |

## 6. Выбор тика: UNION-носитель cmp450_chunk (batch-1, диспатчен)

chunk4⊕chunk5 на свежем master = монотонное расширение сертифицированного RC6-механизма:
send-бурст (join 4 фейк-игроков) = (1) 1× сериализация на чанк-revision (chunk4),
(2) byte[] replay повторных write() (chunk5), (3) section-codec кэши (parse plane).
Банки chunk4-семьи: пары +8.0/+13.0 (2/3), +24.7/+7.3/+13.0 непарные; chunk5: +2.2.
Юнион ≥ изолированного носителя (та же плоскость файлов, без mega-склейки —
урок ×449: цена склейки юниона ~0.3 TPS на 4-плоскостном mega4; здесь +1 плоскость
к chunk4, файлы уже разделены).

Гейты: STRICT-OR cmp437_chunk4 ∨ cmp444_chunk5 ∨ cmp450_chunk на всех 21 rust +
11 java сайтах; marker_id() печатает cmp450_chunk в ARM-строках (evidence grep);
cargo 317/0; check_blobs_sync ALL IN SYNC; javap flat==nested + javap-LOADABILITY
(урок NCDFE ×448) — классы ChunkSendOps/ChunkPacketEncodeOps/ChunkParseOps
загружаются, cp-иглы cmp450_chunk ×3 подтверждены strings/javap.

## 7. Цикл закона 3 дальше

batch-1 (3 ноги cmp450_chunk) vs round-450-anchor-* ×12 (pair Δ≤50k, депресс-гейт
norm≥−2, band 6.0-9.5M). Δ<+20% → batch-2: доп. ноги того же носителя (лотерея
позиций бьётся плотностью, урок ×449: разрыв 181k убил 5 ног) + если пары системно
<+20 — честный LOW-POTENTIAL потолок среза в этой фикстуре (в soak-окне чанк-лейн
= 8.2-8.7% CPU, из которых юнион снимает серилизационный дубль; остаток = moonrise
scheduling + IO-воркеры вне main) → ветка+ноги передаются следующему тику.
