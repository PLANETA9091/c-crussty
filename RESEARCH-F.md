# RESEARCH-F — TASK-434-C (chunk-pipeline R5, закон 6/8, lever cmp434_chunkpl)

Agent: TASK-434-C. Base: origin/round-433-ccefix @513c4835 (CCE-433 фикс внутри).
Вектор: chunk-pipeline R5 — игрокам-видимая ось («чанки медленно грузятся», директива владельца).

## 1. Профильная правда тика-433 (cpu-collapsed, агрегат ×5 якорей @master 8141548)

| lane | якоря (579k) | cce-ноги-носитель (311k) | статус |
|---|---|---|---|
| entityLookup_byChunk (moonrise) | 53834 (9.3%) | 12357 (4.0%) | mob-ось (eindex/soa), не моя |
| paletted read (get/SimpleBitStorage) | 37497 (6.5%) | 18616 (6.0%) | DEMUX REFUTED (запрет-реестр), fluid-bitmask #16 забанкован off |
| tracker (ChunkMap.tick→sweep→sendChanges) | 17186 (3.0%) | 12960 (4.2%) | region-треды уже; остаток = реальная работа пакетов |
| GC (PS*) | 5854 (1.0%) | 2468 (0.8%) | |
| inflate (region zlib) | 4074 (0.7%) | 2389 (0.77%) | boot-ось |
| **parse codec (SerializableChunkData/MapDecoder)** | **4** | **4** | parse-cache УБИЛ |
| recalcBlockCounts | 6 | 5 | dead CPU |
| chunk-send (PlayerChunkSender) | 1 | — | REFUTED 424-C (0.01%) |
| poi | 74 | 40 | dead |
| noise/wgen | 0 | 0 | прегенерированный мир; TASK-429-A жив на cmp429_wgen (+21.8/+23.0) |

## 2. Канон предшественников chunk-оси (git-археология)

- round-419/420-C: block_states parse-cache (cmp420_chunk2) — В КОМПОЗИТЕ, RC6-433: вклад
  +6-8pp в носитель (GC-debt relief). RC3: hit-rate плато 63-65% СТРУКТУРНОЕ (evicted=0 @cap 16384)
  — промахи = первые загрузки, кэшем не лечатся.
- round-421-C (600e3cc): NOISEFILL_ROOTCAUSE — GEN-ось 0.0% inert, parse burst boot-only,
  ramp-шум ±5-8pp.
- round-424-C step-1 (558fd1d = b582bc3): **БИОМС-ПАРС КЭШ** — R5c той эры:
  `ChunkParseOps.parseBiomesSection` (lambda parse7 → mirror template cache + reflection-replica
  MISS + biomes selftest), blob, check_blobs_sync маркеры, cargo test 291/0. **НИКОГДА НЕ МЁРЖНУТ**
  (носитель cmp423_wgen той эры был слаб: soak-медиана ≈+1.2; grep по базе 434-C = 0 совпадений).
- round-424/425-C (5865667): chunk-send slice REFUTED анти-плацебо (0.01%) → boot-Done метрика;
  chunksend-носитель сертифицировал только +17.4 (медиана +8.5) — НЕТ МЁРЖ.
- round-428-C (cmp428_chunkunion): boot-Done parity — chunk-слайс TPS-инертен.

## 3. Выбор подсистемы: CHUNK PARSE/SERIALIZATION — целиком

Три кандидата мандата: chunk-send = 0.01% мёртв (рефьютед ×2); noise-fill = 0.03% inert + занят
живым TASK-429-A (дублирование = сожжённый бюджет); **parse/serialization** — ось чанк-загрузки,
_parse-cache уже в композите (+6-8pp честного маргинала через GC-debt), НО второй decode-сайт
секций (biomes) НЕ покрыт кэшем — каждый HIT по block_states всё равно декодирует biomes
ванильным codec-машинерием (MapDecoder/DataResult/NbtOps + palette machinery).

## 4. Дизайн R5 (cmp434_chunkpl — юнион-носитель закона 7/8)

1. **Порт биомс-кэша** (cherry-pick 558fd1d): parseBiomesSection — зеркальный template-кэш
   биомс-сайта (identity-codec ключ, lock-free probe, reflection-replica MISS = ваниль-декод,
   biomes selftest маркер). Контракт паритета тот же, что у block_states кэша (закон 4).
2. **Ретаг юнион-гейтов**: cmp434_chunkpl STRICT-OR в те же гейты, где живёт cmp430_inside
   (композит) + {cmp434_chunkpl} в chunk_parse/noise_fill/queryplane (чанк-плоскости).
   Носитель = сильнейший композит ⊕ полная парс-подсистема.
3. **Один bulk-JNI/тик**: парс-плоскость = 0 JNI-переходов на горячем пути (документировано в
   chunk_parse.rs; активация = ровно 1 JNI init на бут).
4. Эффект-маркеры: "parse-cache first hit" / "parse-cache selftest PASS" / биомс-"first hit" /
   "biomes selftest PASS" / boot-Done секунды (Done X.XXXs parse).

## 5. Ожидания честно

CPU-потолок биомс-сайта мал (decode_codec 0.05% residual) — маргинал плоскости = аллокационный
(GC-debt relief в burst-окне перезагрузок чанков, класс RC6). TPS-вердикт раунда несёт НОСИТЕЛЬ
(композит 16.5..+23.6 линия ×433 + парс-плоскость). Бар ≥+20% pair min-of-3 = финиш раунда.
