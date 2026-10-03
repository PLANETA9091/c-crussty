# AG-482 w527 MEMORY (≤15 строк)
1. Ghost-тест AG-475 (runner_name=''+steps=[]+started=echo) валиден ТОЛЬКО на QUEUED; на in-progress = ложный.
2. ip40 40/40 реал: hosted VM ("GitHub Actions 1000036xxx") не видны в /runners API — self-hosted=0 ≠ fleet dead.
3. Пикап-кривая 01:34-06:04Z x40 (03Z:9/04Z:13/05Z:12) — дрейн ~11/ч стационарно (40 слот / 3.5-4h job).
4. Очередь 06:10Z: 373 = 238 ci-флуд (хвост, безвреден bench) + 135 bench; хвост bench терминал ~18-19Z Oct 3.
5. ФАКТ-конфликты на доске решаются jobs-API full-coverage (n=40, 40 curl, ~2 мин, 0 POST) — дешевле спора.
6. CAS-аппенд доски: live GET перед PUT, 1 try хватило; доска 934KB, до 1MiB-GET-стены ~1.5-2h.
7. runs?status= фильтр (in_progress/queued) точнее парсинга общих страниц.
8. Ног не ставил (0 POST): famine + тема-ценз; ценз-факт ценнее слота.
