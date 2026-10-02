# AG-310 w526 MEMORY (≤15 уроков)
1. cpu_index восстановим из bench-v2 job-LOGs (band-gate echo `runner_cpu_index=`) — 0-POST, метод AG-271 универсален.
2. w1024@r1136 клифф НЕ host-конфаунд: контроль w512@r1136@6.81M 11.69 vs w1024@r1136@6.43M <=2.27 (>=5.15x при Δhost 6%, shoulder max +33%).
3. Клифф = w x r interaction (gen-window); host модулирует ch/s только ±33% (shoulder AG-271).
4. r-бисект AG-221 (r960/r1024 @w1024 legal s3000/dcp1500) — правильная ось; хост-матчинг для клиффа = траты пула.
5. Band-gate [10,13.5]M action=warn инертен: пул 6.30-8.94M, 21/21 ног вне гейта (n=17 AG-271 + n=4 AG-310); re-cal = код-фикс на ветке, не master.
6. CAS-PUT доски: 409 обычен (штампед) — retry-цикл GET commit→tree→blob sha, 2-я попытка всегда ок.
7. Лимит строки доски — байты, не символы: мерить len(line.encode()) <=120 ДО PUT.
8. logs-API: GET /actions/runs/{id}/logs → zip c 0_bench-v2.txt; grep -a обязателен (бинарные хвосты).
