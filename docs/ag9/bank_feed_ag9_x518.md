# AG-9 ×518 — банк-фид №24 GATE-3: робастная worst-case pair-матем + дедуп-фикс anchor-пула

Метод: normtool_478 (--biomes-exempt; selftest 3/3 bit-exact + 9/9 fixtures PASS),
pair = leg_norm − anchor_norm ≥ +20, Δcpu ≤ 50k, min-of-3 по **РАЗЛИЧНЫМ физическим** якорям,
worst-case = leg − max_anchor в окне (прereg SPEC-AG9-GATE3 ×517). Консервативно: min C55-чтение per run_id.

## 1. ДЕДУП-ФИКС (инфра-урок, material)
Anchor-пул «179» (AG-2 84 ⊕ bank_516 95) содержит **~84 дубли-алиаса**: branch round-513-ax52 ==
run 36723474488, round-512-ax63 == 36708964956, round-512-ax26 == 36708554433 и т.д. (cpu+norm
совпадают до знака). Реальных физических якорей **95**. Все прежние min-of-3, считавшие «3 якоря»
по алиасам — фальсификат: пострадали 36765497995 (1854), P31-IB rep2, 36751906047 (n=3→2).

## 2. Харвест ×518 (мои 2 терминальных SUCCESS из очереди №24)
| run | seed | branch | cpu | norm_v5 | вердикт |
|---|---|---|---|---|---|
| 36765497995 | 1854 | swarm-515-88b @3fefb397 | 6.583M | **+21.59** | CLEAN (но окно: 2 якоря → НЕ min-of-3) |
| 36765494112 | 1853 | swarm-515-88 @3fefb397 | 6.978M | **−0.05** | CLEAN (спред same-пары 21.6пп — σ_seed) |

## 3. №24 GATE-3 — консервативный статус (28 легов × 95 якорей)
- FIRE ≥22.74: 4 лега, top3_min **25.49** (формально ≥3 — как в ×517).
- ROBUST worst-case min-of-3: **2/4** — P31-IB rep1 +25.51 (worst +22.68, n=5), s1670 +25.49
  (worst +22.66, n=4). s1833: ambiguous по C55-чтению (27.85→worst 18.57 / 32.41→23.13);
  s1803 +23.05: fail (worst +15.13). **№24 НЕ закрыт робастно.**

## 4. FIN-кандидаты (worst-case pair min-of-3 ≥+20, MERGE-READY при pair-fresh)
| run | seed | lever | leg | worst pair | якорей |
|---|---|---|---|---|---|
| 36750140047 | 1670 | cmp456_chunkmono_p31snap @3fefb39 | +25.49 @9.017M | **+22.66** | 4 |
| 36752909115 | 1812 | cmp456_chunkmono_p31snap @3fefb39 | +21.25 @6.620M | **+23.00** | 3 (ax63/ax10/ax52, VANILLA, Δ32–43k) |

Честность: +20 только по худшему чтению; ambiguous помечены; pair-fresh/offline-непроверяемо —
за координатором. Полные поллы: docs/ag9/legs_518_ag9_raw.jsonl + gate3_robust_518.json (28 rows).
