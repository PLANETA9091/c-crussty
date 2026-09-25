# RESEARCH-A-438C-CHUNK4 — TASK-438-C (chunk-pipeline WIDENING, закон 3/6/8, carrier cmp437_chunk4)

Agent: TASK-438-C (tick-438, 09:08 +08, Job 406609). Base: round-437-c-chunk4
@7afe6d17 (рестарт застрявшего TASK-437-C: worktree был, коммитов 0 — вектор
cmp437_chunk4 не начат; origin/round-437-c-chunk4 на remote отсутствует, база =
origin/round-436-c-chunk3 tip = тот же sha).

## 1. Мандат

Мандат владельца 2026-09-23: ≥+20% pair-stable ОБЯЗАТЕЛЬНО. Вектор C (закон 8,
игроки-видимая chunk-loading/worldgen плоскость): WIDENING chunk-pipeline —
СЛЕДУЮЩИЙ горячий лист после завершённого RESEARCH-G (parse codec закрыт ×2
сайта; НЕ chunk-parse, НЕ PALETTED — вектор B). Кандидаты задачи:
chunk-serialization снапшот, chunk-send batching, POI-парсинг, light-парсинг;
выбор по профилю (lane ≥2% wall), подсистема целиком (закон 6), флаг-гейт
CRUSSTY_LEVER_FLAG=cmp437_chunk4 (пустой = ваниль бит-в-байт).

## 2. Профильная правда (канон, прегистрированные срезы)

| стадия | вердикт | доказательство |
|---|---|---|
| parse codec (block_states + biomes) | ГОТОВО ×2 сайта | RESEARCH-F: parse 33.38% burst-alloc = codec 19.06 + paletted 13.98; RESEARCH-G: оба сайта кэшированы (cmp434/cmp435) |
| POI | dead 0.01% | tick-433 агрегат ×5 якорей |
| light (parse) | absent | лейн отсутствует во всех cpu-collapsed 421/433/434 |
| serialization (write/autosave) | placebo В SOAK | 300s = 1 автосейв в хвосте (анти-плацебо закон) |
| chunk-send КАК SOAK-LANE | REFUTED ×2 | 5865667 анти-плацебо 0.01%; boot-Done parity ×428 |
| tracker/sendChanges | 2.0-2.2% scene (RECON-36) | network bucket 2.7-2.8% в свежих профилях |
| ramp/GC-механизм | RC6: СТАБИЛЬНАЯ составляющая среза = GC-debt relief | NOISEFILL_ROOTCAUSE.md §2 RC6 / §3 |

## 3. ВЫБОР: chunk-send serialization SNAPSHOT (сериализационная половина
chunk-pipeline)

Почему именно он (по профилю, не по привычке):
1. **Сериализационная половина того же окна.** Parse-сторона (33.38% burst-alloc)
   зеркальна send-стороне: тот же PalettedContainer write/codec-механизм,
   heightmap-NBT, block-entity теги — в том же pre-poll окне (join 4 фейк-игроков
   → forceload → chunk-батчи). chunkpl/chk3 (+14.3..+22.1 norm) живут на
   RC6-механизме (GC-debt relief в population 56s + ранние поллы) — сериализационный
   срез добавляет relief в ТО ЖЕ окно (закон 6: подсистема целиком = обе половины
   chunk-pipeline).
2. **4× дублирование сериализации — структурный дефект ванили.** Kernel ground
   truth: `PlayerChunkSender.sendChunk` строит `new ClientboundLevelChunkWithLightPacket`
   КАЖДЫЙ send = на игрока. 4 фейк-игрока, view-distance оверлап → один и тот же
   чанк сериализуется до 4 раз подряд за секунды (state не меняется).
   Revision-keyed снапшот = 1 сериализация на чанк-revision, остальные — zero-copy
   переиспользование пакета.
3. **Lane ≥2% wall оправдание**: send-плоскость = единственный живой chunk-лейн
   в soak (network bucket 2.7-2.8%, tracker+send 2.0-2.2% RECON-36); снапшот
   снимает сериализационную нагрузку в окне, которое кормит поллы через GC-debt
   (RC6) — механизм, доказанный на ×3 (cp420 vs mg-ноги) и наследованный
   chunkpl/chk3.
4. POI/light — рефьютед-дead (0.01%/absent), write-serialization — placebo:
   строить их = анти-плацебо-риск (дисциплина RESEARCH-G).

Отличие от REFUTED ×2 chunk-send: тот рефьют был про SOAK-лейн (и про
пакет-рейт-рычаг эры 424, +17.4 медиана +8.5 НЕТ МЁРЖ); снапшот не
ре-ранжирует send и не трогает soak-путь вообще — он убирает ДУБЛИ
сериализации в burst-окне (serialize-половина parse-верифицированного механизма).

## 4. Механизм (law 6, подсистема целиком)

- **Хук**: тело static `PlayerChunkSender.sendChunk(SGPL, ServerLevel, LevelChunk)V`
  (точно один сайт посылки чанк-пакетов в ванили) → body-redirect
  `redirect_static_method_body_to_static` на
  `ChunkSendOps.sendChunk` (ТОТ ЖЕ дескриптор — stack-shape contract).
- **Снапшот**: `ConcurrentHashMap<Long, ClientboundLevelChunkWithLightPacket>`
  (key = ChunkPos.toLong, cap 2048 + evict-half — паттерн chunk_parse). HIT =
  запись есть И `!chunk.isUnsaved()` → send(закешированный пакет) — весь
  ClientboundLevelChunkPacketData-экстрактор (buffer/heightmaps/blockEntities) +
  light-пакинг пропущены (zero-copy). MISS/invalid → ваниль-конструкция +
  складирование.
- **Паритет (закон 4)**: пустой флаг = хук не регистрируется (байт-в-байт ваниль);
  любое изменение чанка (блоки/block-entity) = markUnsaved → инвали-дация →
  свежая ваниль-сериализация; shouldModify=true (anti-xray) = bypass кэша в
  ваниль-путь (per-player packet info); события (PlayerChunkLoadEvent) и
  debugSynchronizers.startTrackingChunk сохранены per-send (полная репликация
  ваниль-тела). Остаток (документирован): light-свежесть внутри join-бурста
  (секунды) — консьюмается последующими light-дельта-пакетами (ваниль не
  ре-сериализует чанк-пакет по light-изменению); DEBUG_VERBOSE debug-строка
  пропущена (production-false флаг).
- **selfTest до ARM**: структурный оракул (isUnsaved/getLightEngine/конструктор
  пакета/send/chunkPacketBlockController.shouldModify/события/debugSynchronizers
  резолвятся в pristine-kernel классах) → rust вызывает ДО READY; false = dormant.
  Онлайн-selftest: первые 2 чанка — вторая свежая ваниль-сборка, байт-сравнение
  buffer+heightmaps+light → PASS-маркер.
- **Маркеры**: ARM "[crussty-plugin] cmp437_chunk4: ARMED chunk-send
  serialization snapshot"; EFFECT "chunk4 send-snapshot first hit",
  "chunk4 snapshot selftest PASS", периодические "chunk4 stats serialized:N sent:N
  hits:N". 0 добавленных JNI на горячем пути (план Java-side, 1 JNI static на boot
  — law 6 как chunk_parse).
- **Карриер**: STRICT-OR — chunk_send армится ТОЛЬКО на cmp437_chunk4;
  chunk_parse/noise_fill/etc. добавляют cmp437_chunk4 в свои юнионы (композит =
  все плоскости сразу).

## 5. Потенциал ≥+20%

Носитель (cmp435_chunk3) уже даёт +14.3..+22.1 norm на ногах (chk3-1 +18.7,
chk3-2 +17.0, chunkpl-эра +22.1) — ноги no-pair, вердикт только golden.
cmp437_chunk4 = тот же композит + сериализационная половина окна (4× дубль → 1×)
= монотонное добавление того же RC6-механизма в то же окно + снапшот-экономия
кодек/paletted/light-работы, отсутствующая в носителе. Пайр-верификация —
golden-слот (банкед-диспатчер готов, 3 ноги + 2 якоря @master).
