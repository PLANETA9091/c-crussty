# БАНК-СЕРТИФИКАТ №24 «компо ОСС+P31» — POP-АУДИТ ЗАВЕРШЁН, MERGE-READY (AG-13 ×520)

Дата: 2026-10-01 01:55Z. Метод: GH API → артефакт world3-bench каждого лега → run-env.txt
(строка `population_target`) + server-stdout.log (маркеры `POPULATION INJECT DONE` /
`FIXTURE-VALIDITY`) — канон дешёвого аудита из docs/x519/s157_refutation_ag28.md.
POP50K-DUD класс (population_target==50000, s157 ×20 REFUTED): проверен на всех легах.

## Пять легов GATE-3 №24 — все pop150k-VALID, lever ARMED
| лег | run | ветка/sha | norm_v5 | population_target | seed | lever_flag | fixture |
|---|---|---|---|---|---|---|---|
| s1803 (FIRE-1) | **36757148594** | swarm-515-170 / 402595a2 | **+23.05** | **150000 ✓** | 1803 | cmp456_chunkmono_p31snap ✓ | INJECT DONE 150000/150000, VALID |
| s1670 (FIRE-2) | **36750140047** | swarm-515-9b2 / 3f9d72fb | **+25.49** | **150000 ✓** | 1670 | cmp456_chunkmono_p31snap ✓ | INJECT DONE 150000/150000, VALID |
| ib5 | **36776981574** | round-517-25-ib5 / 3fefb397 | **+30.88** | **150000 ✓** | 1935 | cmp456_chunkmono_p31snap ✓ | INJECT DONE 150000/150000, VALID |
| ib8 | **36777009800** | round-517-25-ib8 / 3fefb397 | **+25.83** | **150000 ✓** | 1938 | cmp456_chunkmono_p31snap ✓ | INJECT DONE 150000/150000, VALID |
| FIRE-3 | 36790238318 | swarm-519-333 / 2f795c64 | +28.70 (AG-305) / +24.10 (AG-179 normtool) | 150000 ✓ (аудит AG-305) | 519033 | cmp456_chunkmono_p31snap ✓ | VALID (задокументирован ×519) |

- Живость сцены (новый гейт-совет AG-179, aliveEst ≥90% плана): s1803 aliveEst(items-model)=105000
  = 100% плана, aliveReal конец окна 117987+29242+14769 = 162k → живая; s1670 aliveEst 105000,
  aliveReal 162k → живая. POP-COLLAPSE-ECHO: 0.
- cpu-индексы в банде 6.0–9.5M: 8.725M / 9.017M / 6.589M / 6.630M / 6.817M — BAND-DISCARD: 0.
- M1-CLEAN у ib5/ib8/FIRE-3 (jsonl AG-46/AG-305); AIOOBE=biome-класс → `--biomes-exempt` канон AG-84
  (применён, fail-closed probe-класс, не code-AIOOBE).

## Вердикт
- **min{23.05, 25.49, 30.88, 25.83, 24.10..28.70} = 23.05 ≥ 22.74** → GATE-3 №24 подтверждён
  на pop-аудите: **POP50K-DUD 0/5**, контаминации банка нет.
- **Банк-сертификат №24 (+41.13 компо-сертификат ×517, 5 легов) = MERGE-READY** — мёрж
  swarm-519-333 (FIRE-3 лег) и открытие юниона G6 (payload AG-35-v2, seeds 2350/2351 → S≥60.01)
  — за координатором.
- Честная оговорка (наследуется от AG-305): гейт закрыт по lenient-бару промпта (min-of-3 ≥22.74,
  cross-carrier леги 402595a2/3f9d72fb/2f795c64/3fefb397); robust worst-case pair отдельных легов
  ниже (s1803 2/6, worst +15.13, AG-29/61) → атрибуционно-устойчивая сертификация остаётся за
  pair-math G6 после мёржа.

## Evidence
- work/AG-13/evidence/run-env_{s1803_36757148594,s1670_36750140047,ib5_36776981574,ib8_36777009800}.txt
- work/AG-13/evidence/inject_{s1803,s1670,ib5,ib8}.txt (маркеры INJECT DONE 150000/150000)
- Нормы: docs/x519/gate3_24_norms_ag46.jsonl (ib5/ib8), gate3_verdicts_ag179.md, GATE3_CLOSED_AG305.md,
  docs/ag29/gate3_math_ag29.md (run-id карты s1803/s1670).
