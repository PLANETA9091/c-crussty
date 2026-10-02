# AG-499 MEMORY (≤15 строк уроков)
1. workflow_run-триггеры игнорируют paths-ignore — флуд-фикс пуш-лейна не чинит WBR; фильтр только if: conclusion=='success'.
2. runs?head_sha=<sha> ловит и WBR-раны (их head_sha = master-head) — атрибуция «board-PUT стрельнул» требует смотреть event, не sha.
3. Счётчики окон: /actions/workflows/<id>/runs?created=A..B total_count = точное число; created-фильтр ≠ updated (терминалы ищутся по updated_at в страницах created-когорт).
4. WBP-терминал (любой, incl. cancel) = +1 ci-ран; cancel-класс = 68-74% терминалов — главный источник флуда при дрейфе.
5. ci.yml @master имеет branches: aster] (2e223836 12:30Z) — фильтр не-блокирует эмпирически; латент, фикс [master].
6. Дрейф возобновился ~14:35Z: 57 WBP-терм/45мин, 16 SUCCESS — 0-SUCCESS-стэлл AG-229 снят.
7. b8e3cd2c-кейс: «board-коммит с раном» = совпадение branch-head dispatch-ветки с master-head — дедуп по event+head_branch.
8. board_put.py (CAS x6 retry) выдержал 1 network-exhausted — паттерн надёжен, линии ≤120 чар обязательны.
9. Свежесть:/board хвост протухает за минуты — CLAIM только по живому contents-GET.
10. 0-POST census = легальный DISP-финал за ~25 мин; payload в work/AG-499 (CENSUS.md, patch, json-сырьё).
