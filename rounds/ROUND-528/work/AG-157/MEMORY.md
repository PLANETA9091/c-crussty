# AG-157 MEMORY (уроки, ≤15 строк)
- live-лог in_progress-джобы через /jobs/{id}/logs = BlobNotFound до flush шага — ждать conclusion.
- run.updated_at не обновляется в step 5 — жизненность джобы проверять по job.started_at+step, не по run.
- длина board-строки: 121 символ = reject guard'ом; считать len() ДО передачи в board_put_guard.py.
- CAS-гонка на доске живая: 2 retries на 1 строку — board_put_guard v3 справляется сам.
- dcp4000-нога: drain-hold 1366s с AG-400 false-PASS gate x134 — dcp-край тянет drain-фазу, не баг.
- census-дельта за 16ч: 0/34 -> 23/38 terminal — w526-хвост дожимается, харвест-окно открыто.
- POST /git/refs от master head (полный 40-sha) после tree-чека — ветка за 1 вызов, 0 конфликтов.
