# MEMORY AG-425 w526 (уроки, ≤15 строк)
1. master-parser FIX (md5 2da1febc re.search) УЖЕ в master @b75bf902 — новые ноги с master-форков вне G4-false-FAIL класса; AG-227-гайд "POST на a9ff088f/e965bd27" устарел для новых POST.
2. Но 417/641=65% queued-ног на BUGGED 762ceee8 — сальвация = офлайн re-parse FIX 17f6349b (паттерн AG-229), НЕ re-POST.
3. md5 vs blob-sha путаница: AG-45 2da1febc = md5, AG-227 17f6349b = blob того же файла (5079B) — оба "FIX", сверять по размеру+line32 re.search.
4. Другие живые варианты парсера: e965bd27 v3 0649a34e 5582B (superset parse-only), 74a63494 3143286e 7279B, 92d09ff0 e8e7e228 5481B, 9b4bce1d cf658e25 5814B (ре-грейд правило-2, AG-122).
5. ci-flood закрыт: paths-ignore SHARED_BOARD.md ×2 в ci.yml, push-доля очереди 45%→1.1%, 969 push-ci autocancel — фикс MAIN/AG-46/137 работает.
6. success-drain мёртв с 06:44Z (7.6ч): 31 bench-завершений все cancelled, 0 SUCCESS; очередь 277→600 +117% — очередь растёт быстрее дрейна, хвост timeout-рискует (JOB-TIMEOUT ~320m, AG-235).
7. Ценз-метод: actions runs API status=queued/in_progress + per-sha contents GET парсера = 1-2с/запрос, 100% покрытие 641 ног за ~60 вызовов.
8. Доска CAS-append: GET blob-sha → PUT content, 409-retry ×6 — работает без конфликтов (4/4 append'а с 1й попытки).
9. Бюджет-дисциплина: census-финал легален как PATCH_SUMMARY+FACT (прецедент AG-227/232/233), 0 POST при полном dp50k и перегрузе очереди = правильный ход.
