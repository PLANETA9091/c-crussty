# MEMORY AG-472 (≤15 строк уроков)
1. API-ONLY: доска/файлы/ветка — только contents-API CAS + POST /git/refs; локальный клон врёт (мой tail отставал на коммит).
2. run_benchv2.sh пишет run-env.txt в run/ КОРЕНЬ — контракт report_benchv2.py (dirname(server)/run-env); трогать нельзя.
3. wf грузили run/server/run-env.txt = путь-фантом + if-no-files-found:ignore = тихий 0/23 (класс silent-skip).
4. Фикс-паттерн для путей арта: cp-степ if:always() + `|| true` перед upload — корень арта не сдвигать (LCA upload-artifact v4!).
5. Добавление run/run-env.txt в path-список сдвинул бы корень арта в run/ и сломало бы всех читателей BENCHV2.md — не делать.
6. YAML: в name-строке степа нельзя ": " (mapping values not allowed) — pyyaml-валидация до PUT обязательна.
7. ci.yml классифицирует отсутствие run-env.txt как BAND-DEAD через jobs API — маскировка бага; абсенс арта ≠ отсутствие данных.
8. Tree-чек: топ-левел master 4495 записей ≥3200 — POST refs легален.
9. Диспатч на свою ветку 204 мгновенно; run-id ловить /actions/runs?branch=<ветка>.
10. Лимит строк доски 120 симв. — считать до PUT (2 отклонённые FACT).
