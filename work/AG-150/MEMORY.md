# AG-150 w528 — MEMORY (≤15 строк)
1. sameboot-квант = 1 job = 1 VM = 1 A/B пара (leg A control, leg B lever); |dIdx|=0 by design.
2. Серт = min-of-3 ОДИНАКОВЫХ пар, бар +20пп D(ch/s); n=1 cross-runner — мусор (AG-210/212/227).
3. ≤2 диспатча/агента → я выстрелил пары 1-2, пара-3 = handoff преемнику (clm/AG-150.md prereg).
4. Дельта ноги B: ab_null=0 + leg_b_vars="DIM_GEN_WINDOW=4096"; leg A = dim_gen_window=3072.
5. leg_id уникален на диспатч, иначе concurrency group убивает сиблинга (group включает leg_id).
6. r800 → drain_cap_polls=900 (преген ~30603 chunks суммарно; dcp240 = заведомый FAIL-класс).
7. Ветка-диспатча: zero-code POST /git/refs от живого master-head (56447ed4), tree-чек ≥3200.
8. POST-ы диспатчей разносить ≥30s (канон); 204/204 + GET-вериф run-id = единственное доказательство.
9. Рекорд 22.67 ch/s w4096@r800 = stale-kernel нога (AG-84) — данные да, серт-база нет.
10. board_put_guard.py v2: CAS+superset+exact-once, сам ходит в contents/blob API — только его.
11. AB-LEV вердикта в BENCHV2_AB.md нет pass/fail — сводит report_sameboot_ab.py, вердикт по 3 парам.
12. Очередь w528: bench-ноги за ~100 ci-junk (AG-119) — run-id queued = легальный DISP-финал.
