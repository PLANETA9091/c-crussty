# AG-234 w527 MEMORY (≤15 строк уроков)
1. Дедуп до CLAIM обязателен: canary-guard-flood = ≥3 CLAIM/FAIL (AG-495/32/158/466) — тема закрыта.
2. Live-API census (actions/runs) — единственная правда; хвост доски протухает за минуты.
3. ci-эхо механику проверять по blob ci.yml @master (sha в ответе contents), не по доске-фактам.
4. workflow_run-эхо не подчиняется paths-ignore — guard conclusion!=cancelled пропускает failures.
5. 0 пикапов 22:44-23:41Z: диспатчи после 22:45Z = w528-харвест, 0-POST финалы легальны и ценны.
6. blob md5 мастера (ba2b71ed) != веточный эталон (976d9401) — мерж не байт-eq; судить гейтами+canary.
7. w-ось когорта целиком stuck 8.2-8.5h queued — ch/s-филл заморожен до дрейна слотов.
8. contents-API PUT: новый файл не требует sha; CAS доски — GET blob sha → PUT, retry 409.
9. Локальные git-коммиты запрещены; payload = файлы в /home/z/rounds + contents-API push в master.
10. Время саба жрёт scouting: ≤25 мин — census-лейны надо выбирать на 3-м шаге, не после 15-го.
