# RESEARCH-459-P27 — Serialization scratch-arena (ID-P27, закон 11 WILD)

Агент: TASK-459-63 · ветка `round-459-p27` (@ origin/master 0d147876) · v18.2.
Идея-карточка: origin/round-458p-ideas:RESEARCH-458-P.md `[ID-P27]`.
Статус: SCAFFOLD (рисёрч + каркас арены + java-stub; активация — отдельный цикл с меркой).

---

## 1. Объект: MISS-путь chunk4/5 и его alloc-burst

Носители (master 0d147876, закон 8 chunk-ось):
- **chunk4** `src/chunk_send.rs` (lever `cmp437_chunk4`) — snapshot-first sender:
  MISS/invalid = точная ванильная конструкция
  `ClientboundLevelChunkWithLightPacket` (section-буферы + heightmap-NBT +
  block-entity tags + light packing), затем снапшот.
- **chunk5** `chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java`
  (lever `cmp444_chunk5`, активатор `src/chunk_send5.rs`) — encode-once/replay:
  **MISS**-путь `encodePayload()` на каждый encode кладёт
  `Unpooled.buffer(256)` (netty unpooled heap, рост удвоением = каскад
  перевыделений) + `new byte[len]` copy-out; online-selftest для первых 2
  MISS-ов добавляет второй полный re-encode (тем же аллокатором).

Профиль (RESEARCH-F, переносим на write-путь): parse codec 33.38% всех
alloc-байт burst-окна; write-путь гоняет ту же PalettedContainer/codec
механику в том же pre-poll окне → serialization-семья = alloc-burst lane.
Прогноз карточки: **burst alloc −15..25% → GC-debt relief; monotone**
(закон-8 ось player-visible chunk-loading, приоритет паритета над wall-clock).

## 2. Идея P27 (механика)

**Арена-пул per-thread** для scratch-объектов MISS-пути вместо свежих
`byte[]`/NBT:
1. per-thread слот арены (region_threads=4 → 4 горячих слота, zero contention
   по построению) с **полной перезаписью** под новым encode (никаких
   частичных дописок поверх старого содержимого) и **длиной-контролем**
   (`write_len` + capacity gate, рост = новый слот, старый на
   переиспользование — канон agrona ExpandableArrayBuffer);
2. section-буферы + heightmap-NBT scratch заполняются в арену; **одна
   защитная копия на выдачу** (`to_owned`/`new byte[len]` ровно один раз) —
   алиасинг-контракт: удержание ссылки кодеком/каналом переживает re-use
   слота только через копию;
3. **байты те же**: арена меняет ТОЛЬКО allocator, не кодировку; parity
   бит-в-бит сохраняется по построению (полная перезапись = каждый байт
   выдачи написан этим encode'ом, длина-контроль = без хвостов прошлой
   жизни слота).

## 3. Web-сайты (≥2, прочитано 2026-09-25)

**[S1] foojay.io — Peter Lawrey, «Java is Very Fast, If You Don't Create Many
Objects», Jun 1, 2023** — https://foojay.io/today/java-is-very-fast/
- «One extra allocation for each event adds 166 ns … reduces performance by
  25%» — цена одного лишнего аллока на событие в high-throughput контуре;
  memory pressure идёт в L1/L2/L3-кэши, а не только в GC.
- «The default behaviour … in Chronicle Wire is to **reuse the same object**
  for the same event type every time … a simple **object pooling** strategy to
  avoid allocations» — прямой прецедент per-type/per-thread переиспользования
  scratch-объектов при сериализации.
- «If this data has to be persisted it must first be **copied**, because
  Objects are reused» — канон защитной копии на выдачу (наш пункт 2.2).
- Бенч автора: GC-время 0.3% при zero-alloc дизайне.

**[S2] aeron.io (Real Logic / Agrona) — «Direct Buffer» docs** —
https://aeron.io/docs/agrona/direct-buffer/
- Триада буферов: `UnsafeBuffer` (fixed-size, «will not be resized …
  IndexOutOfBoundsException»), `ExpandableDirectByteBuffer`/`ExpandableArrayBuffer`
  («When it needs to be resized, a **new byte[] is created and the contents are
  copied** over», default 128 B) — ровно наша модель слота: fixed slot +
  явный grow-протокол вместо неявного удвоения netty.
- `BufferUtil.allocateDirectAligned` / byte-alignment ключевые концепты —
  для цикла-2 (SIMD/скэттер-дисциплина `src/palette_gather.rs`).

**[S3] (контекст карточки) github.com/real-logic/agrona → aeron-io/agrona** —
https://github.com/aeron-io/agrona — «Buffers — thread safe direct and atomic
… with memory ordering semantics»; flyweight/buffer-pool паттерн, которым
карточка мотивирована (/tmp/458p/q14_arena.json, q17_flyweight.json).

## 4. Parity-самотест (re-encode) — контракт

Наследуем online-selftest chunk5 (первым 2 MISS-ам: второй encode сравнивается
бит-в-бит со stash) и поднимаем его на арену:
1. **re-encode parity**: encode A → fresh-alloc byte[]; encode B → арена-слот;
   `Arrays.equals(fresh, arenaCopy)` обязателен (сейчас это vanilla-vs-vanilla;
   с ареной это vanilla-vs-arena — новый смысл того же маркера
   «chunk5 payload selftest PASS»);
2. **длина-контроль**: `arenaLen == freshLen` (нет хвостов прошлой жизни слота);
3. **полная перезапись**: перед каждым encode слот стирается паттерном
   (scaffold-модель: fill+overwrite, прод-версия: write_len-дисциплина),
   выборочная верификация 1/N (паттерн batchOk/nav_plane, one-shot disarm);
4. **fail-closed**: любой дефект (capacity/clone/слишком длинный слот) →
   disarm-латч → MISS-путь навсегда ванильный `new byte[]` (byte-vanilla);
5. **NCDFE-канон**: класс-носитель арены определяется РАННИМ arm-хуком ДО
   первого retransform (паттерн d73758a3/5ecd841a EntityGoalQueryOps);
   NCDFE=0 до вердикта; javap flat==nested; grep AIOOBE=0;
   grep «предлагаю/рекомендую» в вердиктах запрещён (v18.2).

## 5. Scaffold (этот коммит)

- `src/scratch_arena.rs` — Rust-модель арены (std-only, cargo-test-паритет):
  per-thread слоты, полная перезапись, длина-контроль, защитная копия,
  re-encode parity fresh-vs-arena на сидированных payload'ах, disarm-латч,
  счётчики reuse (burst-alloc модель: fresh-выделения −N%).
- `chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java` — stub:
  P27-константы + dormant `p27ArenaSlot...`-протокол длины; горячие методы
  (write/encodePayload/replay/selfTest) НЕ тронуты; **блоб
  chunksend/build/...ChunkPacketEncodeOps.class осознанно НЕ пересобран**
  (shipped cmp444_chunk5 остаётся бит-в-бит прежним; пересборка блоба —
  ОБЯЗАТЕЛЬНЫЙ шаг до любой активации P27, ×93-урок raw-cp маркеров).

## 6. Δ-прогноз

- Lane: serialization alloc-burst (write-сторона ~parse-стороны RESEARCH-F).
- Механика: per-MISS `Unpooled.buffer(256)`-каскад + `new byte[len]` + NBT
  scratch → 1 protective copy; selftest-окно удваивает burst.
- Прогноз карточки: **burst alloc −15..25% → GC-debt relief**; wall-clock на
  bench-сцене (players_packets 0.01%) — monotone-член закон-8 оси, вердикт
  по alloc/GC-debt и pair-stability, не по TPS.
- Потолок цикла-2: SIMD/alignment (S2 BufferUtil.allocateDirectAligned,
  скэттер-дисциплина palette_gather) — отдельная мерка.

## 7. Риски

| Риск | Митга |
|---|---|
| Алиасинг: кодек/канал удержит ссылку на слот дольше encode → порча данных перезаписью | защитная копия на выдачу (S1 «must first be copied»), слот живёт ≤ encode |
| Хвосты прошлой жизни слота (частичная дописка) | полная перезапись + длина-контроль + parity-самотест re-encode |
| Рост payload > слот (лайт-чанки) | grow-протокол agrona-канона: новый слот, старый в пул; никогда in-place resize |
| Per-thread память при region_threads=4 | слоты bounded (cap-гейт), overflow → ванильный путь (fail-closed) |
| NCDFE при определении класса-носителя | ранний define до retransform (канон d73758a3/5ecd841a), NCDFE=0 до вердикта |
| Raw-cp дрейф блоба cmp444_chunk5 | shipped блоб не тронут; пересборка до активации, javap flat==nested |

## 8. URL

- S1: https://foojay.io/today/java-is-very-fast/
- S2: https://aeron.io/docs/agrona/direct-buffer/
- S3: https://github.com/aeron-io/agrona
