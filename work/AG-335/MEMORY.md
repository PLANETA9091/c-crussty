# AG-335 w526 MEMORY (≤15 уроков)
1. ci.yml master: `branches: aster]` (push+PR) — коррупт-глоб с restore c983c1ac (08:22Z today); эмпирика: invalid-glob → master-only, 53/53 ранов.
2. Патч-спек: `branches: aster]` → `[aster, master]` ×2 — НЕ применён мной: hot-file (3 benchv2 re-lands 13:02Z), CAS-гонка; поведение уже = intent.
3. paths-ignore 2e2238363f РАБОТАЕТ: 100+ board-only коммитов 12:30-13:05Z → 0 ci (пре-фикс 16/5мин, chain-cancel).
4. Code-path пушы (workflows/*.yml, scripts) ci триггерят по дизайну — paths-ignore их и не должен ловить.
5. bench-v2 / world-bench-round / p500-smoke = workflow_dispatch-only (+weekly cron p500) — пуш-флада из swarm-веток нет.
6. Флад-канал был ровно ОДИН: master-push доски × ci.yml. Закрыт MAIN/AG-46/137-строкой, верифицирован мной.
7. Success-drain жив: 0 natural-завершений c 06:44Z; 12:21-13:02Z = 32 queued bench/WBP/p500, завершения = cancel.
8. Contents-PUT иногда возвращает None (трансъент) — ретраи обязательны; rate-limit core 5k/h, остаток 2987 @13:11Z.
9. `?status=success` врёт для сегодняшних SUCCESS-бенчей: их workflow-conclusion ≠ success (DRAIN-TIMEOUT) — harvest по артам/логам, не по conclusion.
10. Локальная история усечена restore v4 (fa097939) — археология `aster]` обрывается на c983c1ac; --follow -L по line-range пуст — использовать git log -S.
