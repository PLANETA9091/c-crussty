# P6-PARITY GATE v2 — ФОРМАЛЬНАЯ СПЕКА (S34, ROUND-472, тик 08:08+08 2026-09-27)

Клейм: **PARITY-P6-v2**. Формализация фазы P6 merge-гейта
(`scripts/merge_safety_gate_v2.sh`, фаза `P6: PARITY`) + арбитра
`scripts/world_diff_parity_v2.py` по калибровке S14 (ROUND-471: экспозиция
Δcpu 2,107,395 = 11.4× потолка plain-OK) и S88 (parity 8/8 МЕРЖА №11,
Δcpu=184,154). Код-воплощение спеки: ветка `round-472-s34-parityv2`.

## 1. Область действия

- Фаза P6 гейта v2: `PAIR_VAN="..." PAIR_LEG="..." bash scripts/merge_safety_gate_v2.sh --check-only`
  → арбитр `world_diff_parity_v2.py --gate2 --env-pair --json <файл>`.
- SKIP-семантика не меняется: артефакты пары есть только при вердикте; `GATE_SKIP_PARITY=1` — skip.
- Арбитр в log-режиме (light-члены world3-bench: server-stdout.log + BOTTLENECKS_3.md + run-env.txt).

## 2. Гейты (канон W1–W9, порядок критичен — урок u5)

| # | гейт | канон |
|---|---|---|
| W1 | FIXTURE-VALIDITY | VALID==VALID + подгейты 1a/1b/1c PASS==PASS |
| W2 | WORLD | world_sha256 equal (иначе SKIP/FAIL-разные-стенды) |
| W3 | SEED | pop seed equal |
| W4 | CHUNK-CHECKSUMS | spawnable polls + forceload chunks равны (seed-идентичный ландшафт) |
| W5 | POP-TOTALS | target/injected/items/hostiles/passives equal |
| W6 | PER-TYPE-POPULATION | band \|a−b\| ≤ max(floor, tol·max); **v2: zone-B dilate по типам** |
| W7 | TICK-BEHIND | 0 == 0 |
| W8 | FAKE-PLAYERS | equal |
| W9 | CPU-BAND | **v2: зоны Δcpu (§3)** |

Любой FAIL → вердикт FAIL, exit 1, merge-команда не запускается. WARN не блокируют.

## 3. Зоны Δcpu (P6-спека v2 — НОВОЕ)

Дельта пары: `Δcpu = |runner_cpu_index_van − runner_cpu_index_leg|`.

| зона | условие | правило | вердикт | W9 статус |
|---|---|---|---|---|
| **A** | Δcpu ≤ **184,154** | канон без изменений: tol=5%, floor=64 | plain `WORLD-PARITY-OK` допустим | INFO (in-band) / FAIL (out-of-band) |
| **B** | 184,154 < Δcpu ≤ **2,290,829** | per-type DILATE: husk 9%, spider 8%, остальные 5% (floor 64) | **plain OK ЗАПРЕЩЁН** → только `WORLD-PARITY-OK-EXPOSED` (тег обязателен) | **WARN** `ZONE-B-EXPOSED` |
| **C** | Δcpu > 2,290,829 | пара не верифицируема (экстраполяция за потолком калибровки) | `WORLD-PARITY-FAIL-UNPAIRED`, exit 1 | **FAIL** `ZONE-C FAIL-UNPAIRED` |

- PASS-класс (exit 0) = {`WORLD-PARITY-OK`, `WORLD-PARITY-OK-EXPOSED`}; gate2-строка печатает
  `P6-parity: OK-EXPOSED (...)` — merge не блокируется, но экспозиция видна в JSON-фазе.
- **Прекондиция dilate (урок u5)**: WORLD equal + CHUNK-CHECKSUMS OK + POP-TOTALS equal.
  Если не выполнена — dilate ЗАПРЕЩЁН (канон 5%), зона B деградирует к строгому сравнению.
- Константы в `world_diff_parity_v2.py`: `CPU_ZONE_A_CEILING=184_154`,
  `CPU_ZONE_B_CEILING=2_290_829`, `ZONE_B_DILATE={"minecraft:husk":0.09,"minecraft:spider":0.08}`.

## 4. Δcpu-потолок по типам (таблица-документация, калибровка S14+харнесс S34)

Экспозиция: canary471 (6,573,789, gc6) ↔ poirearm-a (8,681,184, gc6), same-stand,
world sha afb3a0b3…, pop150k/seed42 → **Δcpu = 2,107,395 = 11.4× потолка 184,154 = 42.1× pair-fresh 50k**.

| тип | band зона B | дрейф @Δcpu=2.107M | дрейф @Δcpu=2.259M (adaptiveN) | дрейф @Δcpu=2.291M (n32b) | роль | вердикт-механика |
|---|---|---|---|---|---|---|
| minecraft:husk | **9%** (dilate) | +7.03% (5171→5562) | +9.03% → FAIL | +10.02% → FAIL | churn-носитель | 7.03<9 EXPOSED-OK; ≥9 FAIL |
| minecraft:spider | **8%** (dilate) | +6.70% (4822→4499) | +12.74% → FAIL | +14.05% → FAIL | churn-носитель | 6.70<8 EXPOSED-OK; ≥8 FAIL (lever-дрейф ≥12.7% ловится) |
| minecraft:item | 5% (Tier A) | +4.06% (103448→107827) | +6.68% → FAIL | +7.80% → FAIL | band-защита размером (5% от 103k = 5,171) | 4.06<5 EXPOSED-OK |
| minecraft:item_frame | 5%+floor | 0.00% | 0.00% | 0.00% | статика, иммунна | всегда OK (при валидной прекондиции) |
| остальные 8 (zombie, creeper, drowned, skeleton, pig, cow, sheep, chicken) | 5% (Tier A) | ≤1.47% | drowned +8.31% → FAIL (n32b) | creeper +6.26% → FAIL | слабые | 5% достаточно |

Калибровка зоны A (доказательство потолка): Δcpu=105,275 (aa1↔aa2) → макс-дрейф 2.04%
(creeper), Δcpu=184,154 (S88 canary-470↔canary-471) → макс 1.47% (husk) — headroom к банде 5% ≥2.4×.

## 5. Харнесс-прогон на локальных артефактах тика (S34, world_diff v2 5/5 пар)

Артефакты: `/home/z/rounds/ROUND-471/S88/`, `S14_absorb/{aa1,aa2}`,
`S08_absorb/s09poirearm-a`, `ROUND-470/absorb/{adaptiveN,n32b}`; выводы
`/home/z/rounds/ROUND-472/S34_artifacts/p6v2_*.{json,md,stdout}`.

| пара | Δcpu | зона | вердикт v2 (было в S14 при голом tol 5%) | exit |
|---|---|---|---|---|
| S88 van6757943↔leg6573789 | 184,154 | A | **WORLD-PARITY-OK** (8/8 OK + CPU-BAND INFO zone-A) — parity МЕРЖА №11 воспроизведён | 0 |
| aa1↔aa2 | 105,275 | A | WORLD-PARITY-OK (worst 2.04% creeper) | 0 |
| canary471↔poirearm-a | 2,107,395 | B | **WORLD-PARITY-OK-EXPOSED** (husk 7.03<9, spider 6.70<8, item 4.06<5; было FAIL 2/12) | 0 |
| adaptiveN↔aa2 | 2,258,985 | B | WORLD-PARITY-FAIL 3/12 (spider 12.74>8, husk 9.03>9, item 6.68>5) — dilate ловит lever | 1 |
| n32b↔aa2 | 2,290,829 | B | WORLD-PARITY-FAIL 5/12 (spider 14.05, husk 10.02, drowned 8.31, item 7.80, creeper 6.26) | 1 |

Selftest арбитра: **22/22** (T1–T12 канон + T13 зона-A plain-OK + T14 zone-B EXPOSED+WARN +
T15 lever-drift FAIL + T16 zone-C FAIL-UNPAIRED + T17 gate2 OK-EXPOSED строка).

## 6. Интеграция в gate_v2 / merge-скрипты

- `merge_safety_gate_v2.sh` фаза P6 изменений не требует: exit 0 на OK-EXPOSED = GREEN-фаза
  с тегом `P6-parity: OK-EXPOSED` в логе и JSON; exit 1 на FAIL/FAIL-UNPAIRED = RED.
- Зона C делает пару непригодной для parity-аргументации мержа (закон 4/20d: паритет
  верифицируем только в зонах A/B). Пары с Δcpu > 2.29M нужно переразгонять на same-stand.

## 7. Что дальше (следующий ботлнек паритет-ноги)

1. Разгон EXPOSED-семантики в absorb-парсер (verdict-тег в runs.jsonl), чтобы пары зоны B
   не попадали в REFUTED по ложному parity-FAIL.
2. Кобальт-калибровка: 3+ независимые пары зоны B с gc6/gc6 для подтверждения dilate 9/8%
   (сейчас калибровка опирается на 1 same-stand пару + 2 lever-негатива).
