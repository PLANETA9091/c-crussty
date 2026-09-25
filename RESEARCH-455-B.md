# RESEARCH-455-B — chunk-юнион ребейз-3 поверх диеты-носителя (закон 6+8)

Агент: TASK-455-B · ветка `round-455b-chunk` · носитель: master (после диета-мержа 47ea8b20) ⊕ origin/round-454b-chunk @797ae4f0 (chunk4⊕chunk5⊕chunkparse юнион).
Цель: ≥+20% pair-stable (мандат владельца 2026-09-23). Цикл закона 3: research → имплементация → бенч → Δ<+20% → НОВЫЙ research → доработка → бенч.

---

## 1. Объект ребейза-3: что несёт каждая сторона

**Диета-носитель (47ea8b20, `round-454c-diet`, lever `cmp453_diet`)** — код-носитель диеты-композита:
армит ins4-семью (inside_snap v4 serve-plane, entity_query, mobs_*, goal_selector, colpush, collide_batch, tickplane, queryplane, items_*, stagger) + sense/brain семью (SenseOps getNearestEntity body-swap, BrainOps.tickEachRunning tick2, EntityGoalQueryOps sense-arena) + chunkparse-codec-плоскость. Серт-ноги: diet454-11 +28.3 norm @7310117 (лучший эры), diet454-12 +21.7 @6744409.

**Chunk-юнион (797ae4f0, `round-454b-chunk`, lever `cmp450_chunk`)** — закон-8 ось, 3 chunk-подсистемы + широкий носитель:
- `chunk_parse.rs` (cmp420_chunk2-линия): SerializableChunkData.parse section-decode redirect → ChunkParseOps.parseSection — cache-first decoder (HIT: codec-identity + tag-equality → template.copy(); MISS: reflective twin lambda = ваниль бит-в-байт, parity by construction). TOP-1 alloc-лейн сцены 33.38% ap-burst / 8.1% CPU.
- `chunk_send.rs` (cmp437_chunk4): ChunkSendOps — serialization snapshot (chunk→сеть, batch-encode).
- `chunk_send5.rs` (cmp444_chunk5): ChunkPacketEncodeOps — packet-encode cache (stage-2 поверх chunk4).
- `noise_fill.rs` (cmp419_chunk-линия, TASK-108): DensityFunctions$Noise/ShiftNoise.fillArray whole-body swap → NormalNoiseBatchOps batch fill (G-ABI-2, bit-exact). GEN-ось игрока-видимая (шум генерации).
- Носитель: cmp450_chunk добавлен в гейты mob/tick-стека (sscan, mobpush, mobai, colpush, goalops, entitygoalquery, queryplane, items, entity_query, inside_snap, inside_bitmask, nav_plane, stagger, tickplane, collide_batch) = ins4-носитель ⊕ chunk-плоскости.

Предыдущая мерка юниона: chk454-3 **+15.0 norm** @6530144 → пара +14.1 (↔a9). Ниже бара.

## 2. Ключевой research-файнд №1: GEN-ось (noise_fill) НЕ СТРЕЛЯЛА НИ В ОДНОМ chunk-леге эры

`noise_fill.rs` = two-key gate: lever/env **И** `kernel_policy::decide()` (kernel_policy.rs: `noiseFillArrayWholeBody` НЕ в PROVEN_WINS → strict mode → KeepJava → dormant).
`bench/world3/run_world3.sh` экспортирует `CRUSSTY_KERNEL_POLICY=off` (документированный A/B-оверрайд, kernel_policy.rs: «off = A/B only») только для case-листa `cmp419_chunk|cmp420_chunk2|cmp420_colpush|cmp421_chunk|cmp421_brain`.
Новые chunk-id (cmp434_chunkpl, cmp435_chunk3, cmp437_chunk4, cmp444_chunk5, **cmp450_chunk**) В ЭТОТ case-лист НЕ добавлены → с round-422+ GEN-ось мертва на всех chunk-ногах (регрессия арминга юниона ×454).
**Фикс ребейза-3**: вернуть cmp450_chunk в policy=off case (документированный TASK-108 закон: «арминг под lever в run_world3.sh, пустой флаг = ваниль бит-в-байт, KERNEL_POLICY=off»). Риск 0 (плоскость fails-closed, selfTest-гейт), эффект ≥0.
Механизм: fillArray batch = ОДИН native crossing на массив позиций (G-ABI-2 decoded bit-exact) — закон-6-чистый (bulk-JNI, не per-call). Прецедент ценности: per-call noise bridge REFUTED (WORLDGEN_AB 2026-09-09, +10% медленнее) — именно batch-слой TASK-108 исправляет канал.

## 3. Ключевой research-файнд №2: состав арма cmp450_chunk — недостающая sense/brain семья

В юнионе cmp450_chunk армит ins4-носитель + chunk-плоскости, НО НЕ sense/brain семью (SenseOps.java/BrainOps.java/randomtick BrainOps гейты не содержат cmp450_chunk — см. git grep cmp450_chunk 797ae4f0).
Серт-вектор senseins (cmp451_senseins, МЕРЖ 05c6da1b, min-of-3 +21.9 pair) живёт в master-коде, но на cmp450_chunk-ногах спит.
Цикл-2 (если ребейз-3 < +20%): STRICT-OR-расширение cmp450_chunk в sense/brain гейты (prod + test-helpers синхронно, урок зеркального дрейфа ×451/×452) → cmp450_chunk = полный мега-композит эры (ins4 ⊕ sense ⊕ chunk-плоскости ⊕ GEN) — закон 7 («композиция ВСЕХ реплицированных векторов эры»).
Оценка: chk454-3 +15.0 + sense-дельта (senseins давал +21.9 ПАРУ поверх ins4-носителя) → грубо +20..+30 если аддитивно; субаддитивность ×3 требует мерки, не веры.

## 4. Ключевой research-файнд №3: остаток chunk-оси (закон 8) после юниона

Лейны сцены, остающиеся на chunk-носителях (BOTTLENECK-455 + RECON-29/31):
- **ServerChunkCache scheduling 4.6-5.2%** (chunk-map concurrent-контур + MainThreadExecutor.pollTask + ChunkHolder промоушены) — орchestration-слайс; кандидат цикла-3 (Rust-план фетчей, один bulk-JNI/тик — EPOCH2C-спека §4 M1: Rust владеет снапшотом, Java исполняет).
- **Rust-парс SerializableChunkData (закон-6-чистый)**: текущий chunk_parse — Java-плоскость (кэш + reflective twin). Rust-версия = NBT-section decode в Rust (вход: payload-байты секции, выход: packed voxel/palette), ОДИН bulk-JNI на чанк. Заготовки в юнионе уже есть: parse_diag/zero_cursor/palette_gather/paletted/promote_wire — пре-фаза ребейза-3.
- noise_fill GEN-ось — армится фикс-ом №1 (см. §2).

## 5. План merge (STRICT-OR) и гейты

- База: master ⊇ 47ea8b20 (диета-мерж; таймаут-план: base = origin/round-454c-diet @47ea8b20 напрямую).
- `git merge origin/round-454b-chunk` (797ae4f0). Конфликт-кандидаты: 84 файла (precomputed /tmp/conflict_candidates.txt: 10 .java flag-list, 21 .rs гейт, ~30 .class блобов, run_world3.sh, check_blobs_sync.sh, dispatch/research-скрипты).
- Правило STRICT-OR: в каждом rust-гейте и java flag-list список = ОБЪЕДИНЕНИЕ обеих сторон. Иглы cmp450_chunk + cmp453_diet + cmp436_ins4 + cmp451_senseins + cmp434/435/437/444 + cmp452_mega — ВСЕ сохраняются (зеркальный дрейф ×451/×452: prod-гейты и тест-хелперы синхронно).
- run_world3.sh: объединить lever-case листы + вернуть cmp450_chunk в KERNEL_POLICY=off case (файнд №1).
- Блоб-поверхность: пересборка ОДНИМ javac-проходом (--release 21, cp = round-396-a kernel jar + fastutil [+paper-api/adventure/netty-stub по скриптам]; НЕ round-j2b-jar — урок ×93), NESTED путь первым (include_bytes! контракт), затем flat-копия; `scripts/check_blobs_sync.sh` (flat==nested, major 65, маркеры, флаг-строки в constant pool) ОБЯЗАТЕЛЬНО зелёный.
- Rust: `CARGO_TARGET_DIR=/home/z/rounds/ROUND-455/agent-b/target cargo check --lib && cargo test` (315+), target УДАЛИТЬ сразу (диск-закон ×454).
- Диспатч 2 ног: workflow world-bench-parallel.yml, ref round-455b-chunk-1/2, inputs РОВНО канон (radius 640, seconds 300, fake_players 4, fluid_guard 1, gc_tune 3, inside_cache 1, flush_diet 1, region_threads 4, batch_collector 1, population 150000/42, xmx 10G/xms 4G, band 6.0-9.5M, lever_flag=cmp450_chunk, lever_arg=1; НЕ слать travel_diet/fluid_dirty_ledger; concurrency-гвардов нет).
- Вердикт: пара = leg_norm − anchor_norm, Δ(idx)≤50k, депресс-гейт norm≥−2, band 6.0-9.5M, threw=0, AIOOBE=0, selfTest==true, ARM+ЭФФЕКТ маркеры, NOT-A-BENCH гвард. Цикл закона 3 до ≥+20% или бюджет.

## 6. Бюджет и тайм-план (старт ~07:00 UTC)

- 07:00-07:40: ожидание диета-мержа (полл q3-4мин; timeout → base 47ea8b20) ∥ research + merge-скрипт (этот файл).
- 07:40-08:10: merge + STRICT-OR + блобы + cargo + push.
- 08:10-08:50: диспатч ног 1/2 ∥ параллельно готовим цикл-2 (sense/brain STRICT-OR + возможный ServerChunkCache слайс).
- 08:50-09:30: абсорб, вердикты; <+20% → цикл-2 имплементация + ре-ролл ног.
- 09:30-09:45: RESULT.json, worklog, финал.
