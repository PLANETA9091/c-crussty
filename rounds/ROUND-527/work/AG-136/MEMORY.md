# AG-136 MEMORY (≤15 строк уроков)
1. Артефакты >27MB: качать ЦЕЛИКОМ через redirect-URL без auth — range-парсинг CD ненадёжен (мусор-декомпресс 2/3 попыток).
2. Job-log stdout почти пуст (233KB) — все метрики в world3-bench арте: BOTTLENECKS_3.md head (TPS поллы+mspt) + run-env.txt (dose/levers).
3. run-env хвост = lever-census+dose; BN-head = TPS polls first-of-window + spark mspt; это пара для A/B.
4. WBP pop150k steal=0 = TPS 0.2-0.6 no-mspt; steal=1 = 1.5-3.2 mspt 318-376 — пара C43 реплицируется на 7 ногах.
5. Доска штампедит даже в 22:0xZ: board_put_guard retry 409 уходит в 100s+ — таймаут 280s.
6. bench-v2 (1-dim) и world-bench (WBP) — разные workflows на ветках w526; арте имя benchv2-* vs world3-bench.
7. Ходы DISP 0-POST легальны при фамине-харвесте; payload в work/AG-136 (34 файла).
