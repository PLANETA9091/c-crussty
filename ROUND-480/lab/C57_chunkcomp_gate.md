# C57 — chunk-comp-носитель: ревизия гейта post-№18 (dormancy / residual / ≥2пп)

Тик ROUND-480 (2026-09-28), база 686f2258 (HEAD a372b883), плоскость C57 (ЛАБ chunk-comp).
Прегист-гипотеза (закон 14a, формулировка ДО ценза): «после №18 на chunk-comp-носителе
(3-й серт-мерж эры, round-455, cmp450_chunk-юнион +20.0 min-of-3) существует живой остаток
≥2пп (dormant-плоскость на master-каноне ИЛИ residual-лейн) → диспатч-квота открывается».

## Axis-1 DORMANCY — юнион-ценз гейтов (fresh, c57_gate_census.txt рядом)

- 24 rust-файла несут `cmp450_chunk`; **23/24** армят master-носитель `cmp466_c98ai`.
- **ЕДИНСТВЕННЫЙ дрейф: src/chunk_send5.rs:120-123** (chunk5-encode, LEVER_ID `cmp444_chunk5`):
  гейт = STRICT-eq {cmp444_chunk5, cmp450_chunk, cmp456_poi, cmp452_mega} — БЕЗ `cmp466_c98ai`.
  `git log -S "cmp466_c98ai" -- src/chunk_send5.rs` = ∅ (никогда не добавлялся; cmp452_mega
  добавлен TASK-452-C, cmp456_poi — TASK-456-B; c98ai-эпоха гейт не трогала) = **Л146
  mirror-drift класс** (сиблинг-гейты дрейфуют от юнион-контракта).
- Итог на c98ai-ногах (sensn16-фид, пары №10+): **5/6 плоскостей юниона live**:
  - ins4 (cmp436_ins4): chunk_parse.rs:137 / inside_snap.rs:137 — ✓
  - senseins (cmp451_senseins): mobs_sense/mobs_sscan/mobs_ai — ✓
  - chunk4-send: chunk_send.rs:124 — ✓
  - chunkparse (codec-кэши, ×3 суб-гейта): chunk_parse.rs:138/140/142 — ✓
  - noise-GEN: noise_fill.rs:179+182 — ✓
  - **chunk5-encode: chunk_send5.rs:121 — ✗ DORMANT** («cmp444_chunk5: dormant»)
- Java-блобы chunksend (ChunkSendOps/ChunkPacketEncodeOps): источниковый c98ai_count=0,
  нидлы check_blobs_sync (check_class-блоки :265/:274) без c98ai — синхрон-урок Л146;
  сами нидлы НЕ гейт (ARM чисто rust-side, CARRIER_UNION_450 = evidence-маркер).
- Носитель-класс ЖИВ: R0-верификация Л-479-A2 (run 36374633603 @cmp452_mega) —
  chunk-packet encode cache **ARMED** (L1033/L1045, cap 2048 evict-half, selftest fail-closed).
- Смежный ценз (не дрейф): brainhook.rs TICK2_FLAGS[7] без c98ai — tick2-лейн sense-семьи
  никогда не входил в №9/№10 lineage = консистентный фриз, вне C57-плоскости.
- **НЕ Л212-POI-класс** (0 сертифицированных чисел задето): chunk-эра серты мерились на
  chunk-класс носителях (cmp450_chunk / cmp452_mega / cmp456_poi — все 6/6 армят);
  №9-№18 серты мерились консистентно без encode на ОБОИХ ногах (Δ-cancel);
  canary/health — ваниль (encode не при чём).

## Axis-2 RESIDUAL — лейн-числа на банк-протоколе

- Л190 (канон REFUTED): non-sched chunk-пайплайн на банке мёртв целиком —
  **GEN 0.0%, LOAD-parse 0.0155% wall, SEND ~0пп**.
- Л-480-C20: chunkio-nbt 52.52% = **load-фаза** (a4s2) вне TPS-окна (фазовость ×3);
  steady-state top-30 без chunk-лейнов (всё ≤1.3%).
- Encode-лейн механизм = packet-encode cache (encode-once + byte[] replay per player):
  на банк-стенде 4 fake players / pregen afb3a0b3 → send-объём минимален.

## Axis-3 ≥2пп — capture-матем и вердикт

- best-case капчер encode-лейна на банке ≈ **0пп** (SEND ~0пп) — дефицит к барам
  **≥2.0пп соло / ≥20пп pair конструктивен**; arm-нога «c98ai ⊕ encode re-arm» =
  0-capture-класс S10 → диспатч-квота НЕ открывается (placebo-запрет Л-477-C37.1).
- 19a-стратум (gen_work 9.6% wall) — вне pair-банка (стресс-лестница c/s-метрики).

## ВЕРДИКТ: REFUTED_CENS — 0 диспатчей, 0 код-дельт

Фикс-парковка (hygiene, не диспатч): `+ "cmp466_c98ai"` в chunk_send5.rs:121 +
синхрон нидлов chunksend-блоков check_blobs_sync.sh — riding следующий КОДО-несущий
мерж + canary re-verify (Л145/Л146 канон: гейты двигаются синхронно, bash -n 4-й ценз).
СЕЙЧАС гейт не трогать: sensn16-фид l1a/b/c (c98ai/16) в полёте — mid-window config
drift запрещён (pair-fresh, Δcpu ≤50k FROZEN).

Гейты записи: 0 rust-поверхностей (cargo n/a), band n/a (0 ранов), пороги v5-FROZEN
не тронуты, закон-5 чист (гейты не расширялись, 0 фикс).
Эвиденс: c57_gate_census.txt (axis 1-5), гейт-ценз воспроизводим статикой @a372b883.
