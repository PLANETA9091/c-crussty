# AG-46 ×519 — HARVEST: ib5/ib7/ib8 landed → №24 GATE-3 FIRE-верdict (23:16–23:17Z)

Метод: census sweep (3 pages, created>=18:00Z, snapshot 23:17Z) → 3 новых COMPLETED-SUCCESS
world-bench-parallel ноги refs round-517-25-ib5/ib7/ib8 (sha 3fefb397) → normtool_478.py
(--biomes-exempt canon, selftest 3/3 bit-exact + 9/9 fixtures PASS перед вердиктом) →
lever-атрибуция grep-ом артефактов (server-stdout.log + run-env.txt).

## Вердикты (dud-гейт final_chunks>0 пройден: polls>0, artifacts полные, verdict≠INVALID)
| нога | run-id | ref | norm_v5 | cpu_index | band | M1 | lever | NCDFE | AIOOBE |
|---|---|---|---|---|---|---|---|---|---|
| **ib5 FIRE** | **36776981574** | round-517-25-ib5 / 3fefb397 | **+30.88** | 6588793 | OK 6.0-9.5M | CLEAN | **inside_batch ARMED** (+p31snap stagger) | 0 | biome=2 (probe-класс, exempt по канону ib4/5/6 ×513) |
| **ib8 FIRE** | **36777009800** | round-517-25-ib8 / 3fefb397 | **+25.83** | 6629786 | OK | CLEAN | **inside_batch ARMED** | 0 | biome=1 (exempt) |
| ib7 sub-bar | 36777000445 | round-517-25-ib7 / 3fefb397 | +17.74 | 9048998 | OK | CLEAN | inside_batch ARMED | 0 | biome=2 (exempt) |

Элигибельность: population_target=150000 (НЕ pop50k-echo класс), gc_tune=3, batch_collector=1,
inside_bitmask=0, fake_players=4 — банк-канон env; spark-дивергенция 2.41/9.18/−19.09пп
(report-only). Сырьё: normtool_json_ag46.jsonl (полные JSON).

## №24 GATE-3: FIRE-леги ≥22.74 (min-of-3)
Известные ранее: +41.13 (банк-сертификат AG-2 ×518), топ-3 min 22.77 (ib1-кластер ×517),
base3 +22.74 (AG-14, margin-0.00). **НОВЫЕ ×519 харвестом AG-46: ib5 +30.88, ib8 +25.83.**
- Комбо {+41.13, +30.88, +25.83} → min **25.83 ≥ 22.74 (маржа +3.09)** — даже без банк-сертификата
  {+30.88, +25.83, +23.00 s1812} → min 23.00 ≥ 22.74; {+30.88, +25.83, +22.77} → 22.77 ≥ 22.74.
- **GATE-3 №24 = ЗАКРЫТ устойчиво** (≥3 независимых комбинации, все ≥22.74; margin ≥0.03..+3.09 —
  в отличие от единственного margin-0.00 пути AG-14). №24 → MERGE-CANDIDATE (pair-матрица vs
  95-якорный пул и мёрж = координатор; окна: ib5-пара требует якоря ≤+10.88 в cpu [6538793,6638793],
  ib8 — ≤+5.83 в [6579786,6679786]).

## Прочее со свипа (23:17Z)
- Canary-пара: ОBA QUEUED (36788080912/36788083370, master 876b3f45, created 22:53Z) — pair-math
  bench-v2/S_BV2 по-прежнему закрыт. Press-пары AG-3/AG-13 (36789241460/243676, 36789178921/181296)
  QUEUED; occ-p31 AG-39 (36789584968) QUEUED.
- 5 INFRA-DEAD bv2-ноги AG-41 умерли pre-boot ровно по прогнозу (23:12:36–23:14:11Z, failure) —
  прогноз-канон подтверждён ×1.
- Волна-517-эры wbp-ноги слились в 23:06–23:16Z (9b/29/7b/37b/28-sxh7b/7/29b/4/46a/46b/8/8b/1/18/22
  success) — очередь начала дренаж; STZ-101/прочие 517-ноги = следующие кандидаты харвеста ×520.
