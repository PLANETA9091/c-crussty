# S12: STRUCT-INTEGRITY GATE v1 — preregistered (ROUND-470, ветка round-470-s12-structgate)

МИШЕНЬ: Moonrise #192/#191 (parallel-structgen коррупция) + Paper #14125
(чанк-ген НЕ чистая функция сида → world_sha256-гейты слепы при region_threads>0,
канон **0/169**). Статус: СТЗ-дизайн + вайринг-стаб `scripts/struct_integrity_gate.py`
(selftest 8/8 GREEN офлайн; CI-нога не диспатчена — вайринг, не lever).

## 0. Preregister-канон (фикс до первого лега; менять после — запрещено)

| # | гейт | требование |
|---|------|-----------|
| G-S1 | SEED-IDENTITY | level.dat seed (WorldGenSettings.seed / RandomSeed) равен обеих сторон; иначе `STRUCT-NOTCOMPARABLE` (мир не судим — судим сцену) |
| G-S2 | DETERMINISM | повтор серийной стороны обязан 169/169; иначе env недетерминирован → **REFUTED-CENS abort** (вердикт lever'у запрещён); repeat=2/сторону = 4 digest-прохода/вердикт минимум |
| G-S3 | CHUNK-PARITY | семантические per-чанк дайджесты probe-окна **169/169** (13×13, r6; бандов нет — структурная коррупция = коррупция); отчёт top-K (cx,cz) обязателен |
| G-S4 | STRUCT-MARKERS | дайджест structure-starts (root["structures"]) равен; односторонний старт = FAIL (потерянный старт при целом ландшафте виден ТОЛЬКО здесь) |
| G-S5 | BLIND-RULE | world_sha256 / raw-NBT дигесты НЕ являются pass-доказательством; только теневая метрика blind_raw |

## 1. ЧТО хешируем (canon дайджеста)

- `semantic(c) = sha256("S12v1|{cx},{cz},{status}" + Σ sorted sections "{y}:sha256(block-id sequence)")`,
  где block-id sequence = `|".join(palette[unpacked_idx[i]] for i in 0..4095)`.
  **Семантический** = инвариантен к reorder палитры/упаковке (бенигн-недетерминизм
  Paper #14125 и летучее поле @92 — Л199), чувствителен к любому изменению блока.
- `raw(c)` = parity-v2 `_canon_digest` (палитра-строка + data longs) — ТЕНЕВОЙ
  счётчик blind_raw, никогда не pass-доказательство (G-S5).
- markers: starts (id, ChunkX/Z, pieces, bbox) + References → sha256.

## 2. КОГДА (фазы съёма)

T0 gen-from-scratch, `Status==full` (ПОСЛЕ placement структур), ДО сейва и ДО тиков
(тик-плоскость мутирует блоки — она покрыта parity-v2 P6, не этим гейтом).
CI-протокол: две ноги gen-from-scratch (region_threads=0 серийная vs lever),
один сид, repeat=2, артефакты = оба мира + report. Интеграция: фаза **P7-STRUCT**
после P6-PARITY (SKIP-able, как P6) либо standalone до мерджа structgen-рычага.

## 3. КАК паритетим (связка с канонами)

- Seed-идентичность: G-S1 (seed из level.dat) — предусловие, не результат.
- Population/entity-плоскость остаётся за world_diff_parity_v2 (канон b3853246,
  selftest 17/17 подтверждён в этом тике); S12 добавляет **block-семантику +
  structure-markers** — lanes, которых в parity-v2 нет.
- world_identity-канон Л199 (sha256(world_sha‖config‖SAH-order), 9/9 GREEN) остаётся
  world-уровнем; S12 = недостающий per-чанк уровень локализации (0/169 → 169/169).

## 4. Числа стаба (selftest 8/8 GREEN, офлайн, region-фикстура 700416 B = 1 mca)

| T | сцена | вердикт-числа |
|---|-------|---------------|
| T0 | pack/unpack roundtrip | 4096 idx == |
| T1 | identity/повтор | 169/169 OK, blind_raw 0/169, ~1.1s |
| T2 | блок-коррупция (0,0) | **168/169** RED, локализация (0,0), wdp2 blocks_diff 1366 |
| T3 | односторонний structure-start (1,1) | content 169/169 НО markers DIFF → FAIL (Moonrise #192 класс) |
| T4 | бенигн reorder палитры (169 чанков) | semantic 169/169 OK; **raw слеп 169/169** (100% ложных флагов) |
| T5 | reorder+коррупция (3,−2) | semantic 168/169 + (3,−2); raw 169/169 — 0 бит локализации |
| T6 | seed 470≠471 | STRUCT-NOTCOMPARABLE |
| T7 | fixture | 169 чанков = 1 region 700416 B, gate ~0.9s |

REFUTED_CENS-квант: raw/world-sha гейт как pass-оракул REFUTED — ложный-RED 169/169
на бенигн-reorder классе + 0 бит локализации; потолок замены = 169/169 локализаций
ценою ~0.9s/вердикт (169×4096 unpack+sha256 ×2 стороны).
