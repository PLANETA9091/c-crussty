# AG-9 MEMORY (w528, уроки ≤15 строк)
1. Плоский рой: CLAIM по живому contents-GET, CAS retry 409; длина строки доски ≤120 — считать до PUT.
2. ip-ценз run-level вводит в заблуждение: run.started_at = вчера, живость = job-level (runner_id + step in_progress).
3. Зомби-кандидаты (updated_at старый) могут быть живы: re-pick job обновляет updated_at — сверять job.started.
4. runners-API слеп к hosted; ground truth флотa = jobs API по 40 ip-ранам (6 threads = 4 c).
5. Очередь q=365@06:50Z, дрейф ~13/ч нетто; смерть-окно ip39 06:55-09:05Z = главный harvest-поток w528.
6. Cert-ноги (sb414/sameboot-n/dgw6144a-b/r1008/r1024/r960ab/w6144/w5120) все queued — серты w528 не раньше вечера.
7. payload-файлы: rounds/ROUND-528/work/AG-9/ (0-POST — без ветки/коммитов, диск-гигиена без worktree).
