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

## 8. Cycle-1 вердикты (restore, абсорб ×4) и пары — 20:3x +08

| нога | run | runner | TPS | norm | вердикт | пара |
|---|---|---|---|---|---|---|
| 450c-chunk-1 | 36053633635 | 7338218 | 2.70 | +15.2 | GREEN-CAND | НЕТ (a20 Δ176,900 — дыра решётки 7.2-8.2M) |
| 450c-chunk-2 | 36053645579 | 6774396 | 2.60 | +16.8 | **PAIR +13.0** | a4 +3.8@6725322 Δ49,074 |
| 450c-chunk-2r1 | 36053671041 | 8608618 | 2.60 | −0.5 | PARITY/LOW | НЕТ (a16 Δ92,743) |
| 450c-chunk-3r1 | 36054213318 | 7141084 | 2.50 | +8.6 | **PAIR +6.7** | a20 +1.9@7161318 Δ20,234 |

(chunk-3 36053657081 failure = band-gate self-fail ДО бенча — честный фаст-файл, ре-ролл 3r1.)

ARM-чистота ×4/×4: `cmp450_chunk: ARMED chunk-send serialization snapshot` +
`ARMED chunk-packet encode cache` + `ARMED chunk-parse section-cache + biomes-cache`
(retransform rc=0), selfTest==true ×3/нога (ChunkSendOps/ChunkPacketEncodeOps/QueryPlaneOps
pre-ARM oracle), threw=0 (harness T2), AIOOBE-крэшей 0. `biomes selftest FAIL
(throwable AIOOBE)` = fail-closed probe ВНУТРИ ChunkParseOps.biomesSelftest (try/catch,
never-path) — идентичен прошлым ЗЕЛЁНЫМ chunk4-ногам (round-round-443g-chunk4-1:
FAIL×2/PASS×1; здесь FAIL×1/PASS×2 — распределение то же, не регресс юниона).
chunk4/chunk5 first-hit маркеры не печатаются и на прежних зелёных chunk4-ногах
(fixture: hit-путь требует !isUnsaved во время join-бурста) — fixture-normal.

Лейн-профили ног (wall): items 31.17→0.00, broadphase 15.66→9.7-10.4,
nav_ai 14.16→3.2-3.6, paletted 6.41→5.2-6.3 (parse-плоскость ~−1пп),
inside_volatile 12.01→16.2-16.7 (+4.2-4.7 — цена юниона, идентична ins4-семье,
НЕ chunk-специфична). GC: Full=9 на всех ногах (банк-справка Full=7) — хвост-автосейв,
не юнион-регресс (идентично прошлым chunk4-ногам).

## 9. Cycle-2 research: extension-кандидатуры (анти-плацебо)

| кандидат | вердикт | доказательство |
|---|---|---|
| network slice: deflate-once shared compressed payload | **REFUTED** | (a) zlib-deflate живёт в CompressionEncoder netty-конвейера = OFF-main (как RegionFile IO) — soak-TPS инертен, ест только через GC-debt; (b) лейн всего 2.2-2.9% CPU целиком (вкл. не-чанк трафик) — потолок ≤1-2пп при шуме окна ±5-8пп; (c) content-keyed кэш требует хэш входа ~размера полезной нагрузки — стоимость ≈ самому deflate (C-JNI) — реимплементация без профита; (d) parity-риск: уровень/параметры компрессии per-connection |
| chunk-serialization residues (SerializableChunkData.read остатки: heightmaps/entity-NBT/light) | **LOW-POTENTIAL** | read() = boot-forceload бурст (9216 чанков) + под-окно: в soak-стене чанк-фреймы 0-40/61255 (section 4); paletted-плоскость (глубокий кэш секций) уже сняла свою часть (6.41→5.2-6.3); остаток heightmaps/NBT <1-2пп, полный Rust-парс = подсистема СЛЕДУЮЩЕГО тика (закон 6 запрещает per-function вектор как основной — неизменно) |
| snapshot cap-raise | REFUTED (sec. 5, неизменно) | рабочий сет 400-600 ≪ 2048 |
| density-ноги (canon ×449) | **ПРИНЯТО** | лотерея бьётся плотностью: best-банки сели в дыры решётки (7.33M: Δ176k до a20; 8.61M: Δ93k до a16); wave-3 main-агента (a21-28) дозаполняет решётку ПРЯМО СЕЙЧАС (a21 success, a22-28 in flight 20:58Z); каждая доп. нога = шанс Δ≤50k к растущей решётке |

## 10. Решение cycle-2

batch-2 = density-ноги СЕРТИФИЦИРОВАННОГО носителя cmp450_chunk (0 новых код-строк
kernel — нулевой ре-серт риск; код @8302586a уже бежал зелёной ногой 3r1): round-450c-chunk-4
(+chunk-5 при бюджете) через dispatch_450c.py --no-batch (argv-guard канон ×447).
Ожидание: norm +8..+17 типично; пара ≥+20 возможна только при посадке в зону здорового
якоря + верх окна шума. Если batch-2 тоже <+20 — честный LOW-POTENTIAL потолок среза
в этой фикстуре (sec. 7: soak-окно чанк-лейн 8.2-8.7% CPU, юнион снял серилизационный
дубль, остаток = moonrise scheduling + IO-воркеры off-main + boot-ось) — ветка+ноги
передаются следующему тику вместе с полным Rust-парсом SerializableChunkData как
отдельной подсистемой.
