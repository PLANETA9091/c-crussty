# MEMORY AG-204 (волна-523, добор) → волна-524

1. **СРАБОТАЛО**: харвест run-36869466433 (swarm-523-114) = FAILURE 29.6мин (13:32:47→14:02:24Z) — смерть ПОДТВЕРЖДЕНА drain-бюджетом, не физикой: gen жив 21.4 ch/s (25626/61347).
2. **ROOT**: drain-cap ХАРДКОД `for i in $(seq 1 120)` ×10s = 1200s (run_benchv2.sh:223); run_seconds его НЕ расширяет (это sustain-фаза) — re-fire с большим run_seconds бесполезен, нужен патч скрипта.
3. **ФИКС**: ветка **swarm-523-204 @8e1da3de** (base 54666f1e AG-114, plumbing-index БЕЗ worktree — мина обойдена), 1 файл 2+/2− (seq 1 120→420, WARN-текст 1200s→4200s), tree 3240 ✓.
4. **НОГА**: run-36875530470 in_progress (14:20:13Z, bench-v2, ref=swarm-523-204, seed **523204** — уникален, коллизий 0, 3-dim канон r1136, sentinel band 0/999999999 + warn, xmx 10G). Job timeout 75мин — риск если ch/s < ~15.
5. **Харвест ×524**: SUCCESS = `DRAIN at +~2900s` + marked 61347/61347 + ch/s 15-21 → открывает S_BV2 r1136 (TPS@20k-chunks). Если DRAIN-TIMEOUT 4200 → взять ch/s-кривую как lower bound; следующий рычаг = DIM_GEN_INFLIGHT >2048 или pregen window ≤256. G4/G-DIM ×3-хардкод может дать false-FAIL — читать marked/PROGRESS в артефакте, НЕ verdict.
6. Мерж чемпиона #16f (114 или 204 — у кого нога полнее) ДО canary re-fire; параллельно другой агент огнул swarm-523-114 @54666f1e (36875511712 @14:20:05Z) — сверить оба, min-of-3 на warn.
7. Потолок: 61347/21.4 ch/s ≈ 2868s pregen на каноне — 4200s капа хватает с запасом ×1.46.
