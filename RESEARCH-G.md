# RESEARCH-G — TASK-435-C (chunk-pipeline R6, закон 3/6/8, carrier cmp435_chunk3)

Agent: TASK-435-C (tick-435). Base: origin/round-434-c-chunkpl @a17cde0b (обе
section-decode кэша + cmp434_chunkpl внутри).

## 1. Мандат цикла закона 3

Закончил x434 ниже пары (+21.9 norm chk-3 — high-зона 8.4-8.9M без здорового якоря).
Цикл: НОВЫЙ рисёрч по следующему chunk-листу (parse-стадии: POI? light?
serialization? chunk-send?) → подсистема целиком (закон 6) → новый STRICT-OR
флаг cmp435_chunk3 ПОВЕРХ cmp434_chunkpl.

## 2. Вердикт по каждой стадии (профильная правда, канон)

| стадия | вердикт | доказательство |
|---|---|---|
| parse codec (block_states + biomes) | **ГОТОВО** (обе сайты кэшированы x434) | RESEARCH-F: parse 33.38% burst-окна = codec 19.06 + paletted 13.98; RC3: hit-плато 63-65% СТРУКТУРНОЕ, evicted=0 @cap 16384 |
| POI | dead — НЕТ плоскости | tick-433 агрегат ×5 якорей: 74/40 сэмплов ≈ 0.01% |
| light | absent — НЕТ плоскости | лейн отсутствует во всех cpu-collapsed таблицах тиков 421/433/434 (микро) |
| serialization (write/autosave) | placebo — НЕТ плоскости | бенч 300s = 6000 тиков → 1 автосейв в хвосте окна; анти-плацебо закон |
| chunk-send | REFUTED ×2 — ЗАПРЕЩЕНО каноном | 5865667 анти-плацебо 0.01%; chunksend-носитель +17.4 медиана +8.5 НЕТ МЁРЖ; boot-Done parity ×428 |
| region read/inflate | REFUTED RECON-35 | block-read семья 7.5% soak → bench-inert |
| paletted read 6.0-6.5% (жив) | нет легального соло-рычага в бюджете | DEMUX REFUTED-BY-ECONOMICS соло (§148-149, субстрат верифицирован для combo); fluid-bitmask #16 забанкован off |
| tracker/sendChanges 3.0-4.2% | real packet work; region-треды уже | не трогаем в этом раунде (инстанс-редирект тулинг отсутствует) |

## 3. Вывод (честно)

Chunk-pipeline подсистема ЗАВЕРШЕНА на гранулярности закона 6: обе
section-decode сайты = 33.04 из 33.38% burst-аллокаций покрыты кэшами;
остаточные стадии profile-dead / canon-refuted / забанены. НОВАЯ плоскость
этим тиком = местоbo-риск — НЕ строим (дисциплина анти-плацебо ×2).

## 4. Деливери R6: cmp435_chunk3 (STRICT-OR поверх cmp434_chunkpl)

1. Ретаг ×28 гейт-сайтов (19 rust + 8 java + ColpushOps FLAG5 +
   ChunkParseOps CARRIER_UNION_435 + check_blobs_sync маркер) — тот же
   юнион: композит (cmp430_inside set) ⊕ block_states кэш ⊕ biomes кэш.
   Round-id hygiene: сертификационные ноги ROUND-435 несут id раунда.
2. Блобы: build_chunkparse_ops + build_430b → flat==nested OK;
   check_blobs_sync ALL IN SYNC; javap CARRIER_UNION_435/FLAG5 видны.
3. cargo test 303/0 (needle-тест расширен: blob cp несёт cmp435_chunk3).
4. Fail-closed: пустой/чужой флаг = ваниль (дормант-инвизибл), закон 4.

## 5. Кандидаты СЛЕДУЮЩЕГО углубления (вне бюджета тика, задокументированы)

- (a) cross-process template persistence (CDS-класс) — лечит 35-37%
  first-load промахи при повторных прогонах; новая подсистема хранения.
- (b) paletted guard-v2 relens + ni комбо на верифицированном субстрате
  (§150 lane) — mob/axis пересечение, координация с владельцем лейна.
- (c) tracker/sendChanges slice — требует virtual-redirect тулинг
  (redirect_method_body_to_static не использовался на Paper 1.21.10
  инстанс-методах чанк-трекера; отдельный раунд-раунд RECON).
