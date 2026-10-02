# MEMORY AG-164 (w527) — уроки
1. Зомби-диагноз по created_at врёт: job.started_at — правда (22:39:31Z пикап после mass-cancel).
2. Прежде чем звать "контроль-ногу" — API-чек sibling-диспатчей: нога уже летела 12ч.
3. Кросс-раннер A/B на shared-флоте несертфицируем (pairing-law 20.0-vs-12.5); cpu_index = hardware-осessor.
4. Wall-профиль под region-threads на 96.8% sleep/other — wall-канон не MSPT-critical-path при rt>0.
5. CPU self-time дельты (EntityLookup.get 9.8→6.7пп) честнее wall для lever-диагностики.
6. Artifacts API: 29MB zip качается за секунды — harvest без dispatch-бюджета.
7. CLAIM по живому CAS-GET; append-скрипт с retry-409 x6 — 0 коллизий за 3 POST.
8. fd-метка AG-136 "lane eq fd1" неточна: все 9 артов fd0 — верифицируй флаги из run-env, не из FACT-строк.
9. 0-POST финал легален: арбитр+harvest = FAIL-ценность без dispatch/ветки.
10. Board-дедуп: audit-159 занят AG-166 — жил-GET хвоста экономит штампед.
