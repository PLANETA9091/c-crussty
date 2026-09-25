# RESEARCH-459-P23 — RegionFile sector-read pool + read-ahead (ID-P23, TASK-459-61)

Агент: WILD закон-11 тик-459 (v18.2). Идея: [ID-P23] из RESEARCH-458-P.md (ветка round-458p-ideas).
Статус: SCAFFOLD (v1 = control-plane в Rust + dormant java-hook; чтение файла остаётся ванильным
FileChannel до волны 2 — см. «Риски»).

## 1. Механика (что и почему)

RegionFile-IO burst — ванильный путь читает чанк-пейлоад из регионального файла посекционно:
на каждый чанк минимум 1 pread заголовка-локации + 1..N pread 4KiB-секторов тела
(len + compression + данные) через `FileChannel.read`. При reload-сцене (burst группы чанков)
запросы идут группой в ОДИН регион → соседние чанки лежат в соседних 4KiB-секторах →
классический read-ahead-профиль (prefetch на java-heap уровне с контролем аллокации).

План v1 (этот scaffold):
1. **Пул 4KiB-секторных буферов** (java-heap): фиксированный free-list `byte[4096]`,
   выдача = КОПИЯ (пул НЕ алиасится — одна копия на выдачу, карточка P23),
   возврат = переиспользование.
2. **Read-ahead соседей региона**: при батч-вызове на группу чанков строится план
   (region → {chunk → диапазон секторов из локаций-таблицы}), 8-соседство на чанк-сетке
   префетчится в пул ДО запроса.
3. **Java читает из пула** (бридж-хук в ChunkParseOps-плоскости — точка входа карточки),
   **miss → ванильный FileChannel** (fail-closed: любой ERR/капа/недоверенный диапазон → ваниль).
4. **Parity-контроль бит-в-байт**: формат на диске НЕ меняется; над полученными байтами
   считаются `len` (4B BE int поля) + CRC32 и сверяются с референсом ванильного чтения
   на selftest-окне (chunk4/5 selfTest-дисциплина). Пул только читает — байты не мутируются.

Rust-сторона (src/region_io.rs, v1): control-plane — модель заголовка региона
(1024 локаций: 3B offset-in-sectors + 1B sector-count; 1024 таймстампов), план read-ahead
(соседство → секторные диапазоны), пул секторов (free-list + no-alias handout), len+CRC32
верификатор, флаг-гейт + ARM-маркеры. JNI-бридж (bulk: список чанков → битмапа HIT/MISS +
данные) — волна 2, контракт зафиксирован в javadoc модуля.

NCDFE-канон: будущий бридж-класс не имеет вложенных классов (ZERO nested classes),
определяется EARLY в kernel loader вместе со всеми nested-blob'ами — протокол round-3
(run 35902792520): каждое `$Nested` из констант-пула должно быть в списке define_class.

## 2. Сайты (что перехватывается)

- **Чтение**: ванильный `RegionFile.read(ChunkPos)` → `DataInputStream` на секторный диапазон;
  в Moonrise-патченных кернелах путь идёт через `MoonriseRegionFileIO` (per-world
  RegionDataController, task-очереди per-region, приоритеты BLOCKING/HIGHEST) — хук ставится
  ВЫШЕ очередей (на границе запроса группы чанков), координацию задач Moonrise не дублирует.
- **Точка входа java**: `chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java`
  (уже в kernel loader, law-7 union с chunk-осью) — v1 добавляет dormant-хук
  `regionSectorPoolGet` (HIT из пула / MISS-константа → ваниль).
- **Не трогаем**: запись (RegionFile.write — flush-плоскость, отдельный риск), .mcc external
  чанки (v1 miss → ваниль), RegionBitmap.

## 3. Parity (байты бит-в-байт, len+CRC)

Формат (спецификация + javap кернела @round-396-a) — пул его не меняет, только читает:

```
Header: 2 сектора по 4096B:
  [0..4096)     location table: 1024 x BE int32 — верхние 3 байта = offset в секторах
                от начала файла, младший байт = размер чанка в секторах (макс 255)
  [4096..8192)  timestamps: 1024 x BE int32
Payload (по offset*4096):
  [0..4)        length (BE int32) — длина ОСТАЛЬНОГО тела, паддинг НЕ включён
  [4]           compression (2=Zlib по умолчанию; 3=none; 4=LZ4; 127=custom+.mcc external)
  [5..4+length) compressed NBT; файл добивается 4096B-паддингом (валидатор кернела
                отвергает непаддингованный хвост)
```

javap кернела (patched-kernel.jar round-396-a) подтверждает поверхность:
`RegionFile.SECTOR_BYTES (private static final int)`, `SECTOR_INTS`,
`EXTERNAL_FILE_EXTENSION` (.mcc), `EXTERNAL_STREAM_FLAG`, `EXTERNAL_CHUNK_THRESHOLD`,
`RegionBitmap usedSectors`, `roundToSectors(long)` — v1-пул сверяется с этими же инвариантами
(len + паддинг + границы секторов), selftest-окно (первые N секторов после ARM) сверяет
байты пула с ванильным FileChannel-чтением по CRC32 (crc32ieee) — расхождение = DISARM навсегда.

## 4. Δ (прогноз)

- reload-ось: **+0.5-1пп** (burst-чтение групп чанков: пул-хит дешевле системного чтения +
  меньше ожиданий page-cache; на soak-CPU инертно — carrier-член оси).
- GC-debt: relief — переиспользование `byte[4096]` вместо per-read аллокаций в буферах
  декодера (нижняя граница эффекта P24/P27-семейства, на IO-плоскости).
- Гейт-метрика (волна 2): hit-rate пула >= 60% на reload-сцене — ниже = DISARM.

## 5. Риски

1. **FileChannel в нативе опасен** (карточка): v1 НЕ открывает файл в Rust; read-ahead только
   в java-heap пул, все fd остаются на java-стороне.
2. **Алиасинг**: выдача = защитная копия (одна копия на выдачу); пул никогда не отдаёт
   внутренний буфер по ссылке.
3. **Stale-сектора**: регион может быть перезаписан между префетчем и чтением — валидация
   (region, offset, len, CRC) + эпоха файла (mtime-контроль java-стороны); несовпадение →
   miss → ваниль.
4. **NCDFE**: бридж без nested-классов, EARLY-define; javap-loadability gate
   (check_blobs_sync.sh) перед dispatch.
5. **Moonrise-расхождение**: хук на ChunkParseOps-границе (union с chunk-осью), очереди
   Moonrise не затрагиваются.
6. **Память пула**: кап фиксирован (дефолт 4096 секторов = 16MB java-heap, константа модуля),
   overflow → miss → ваниль, эвикции нет.

## 6. Источники (URL)

1. https://github.com/Tuinity/Moonrise — source-контроль IO-плоскости:
   `ca/spottedleaf/moonrise/patches/chunk_system/io/MoonriseRegionFileIO.java` (mc/1.21.4,
   1700 строк: RegionDataController per world, chunkTasks CHM по chunk-key, prioritised
   per-region task queues, getIOBlockingPriorityForCurrentThread = BLOCKING/HIGHEST,
   CancellableReads, ReadData{result, input, syncRead}) — fetched raw.
2. https://wiki.vg/Region_File_format — контент перенесён в
   https://minecraft.wiki/w/Region_file_format (fetched via reader: 2 сектора заголовка,
   1024xBE32 location (3B offset + 1B count), 1024xBE32 timestamps, payload
   len+compression+data, 4096B-паддинг, compression 1/2/3/4/127, .mcc >1020KiB).
3. (локальная парити-грунда) javap net.minecraft.world.level.chunk.storage.RegionFile
   @research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar — SECTOR_BYTES/SECTOR_INTS/
   EXTERNAL_STREAM_FLAG/EXTERNAL_CHUNK_THRESHOLD/RegionBitmap/roundToSectors.
