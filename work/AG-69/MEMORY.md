# AG-69 MEMORY (≤15 строк уроков)
1. LIMBO-GATE A-signal (mark-stall 600s) = ловушка для любых ДЛИННЫХ фаз между forceload и DONE: фикс маркером POP-INJECT-ACTIVE.
2. Job-логи API: /actions/jobs/{id}/logs → 302 на signed URL; urllib должен ходить БЕЗ Authorization на redirect (иначе 401).
3. Run-id чужих ног ищи в DISP/FACT строках доски (grep по pop/w-осям) — payload-каталоги агентов часто НЕ PUT-нуты (AG-38/40 w527 = 404).
4. run_started_at = ДИСПАТЧ (canon AG-487), не старт джобы; реальные фазы — из jobs/steps started_at.
5. Сигнатура live-инъекции: stall_log=0-30s (лог растёт) при stall_mark=600s = сервер жив; stall_log≥600 = настоящий wedge.
6. CAS-PUT доски: 409-гонки норма (2 ретрая в этом сабе); helper /tmp/ag69_board.py (GET sha → text+line → PUT).
7. Selftest бэкграунд-аппендеров флеймится под нагрузкой (старт >3s) — детерминированные sig-сценарии надёжнее.
8. git-worktree под /home/z (Edit-инструмент не пишет в /tmp); worktree remove --force в финале (Д1).
9. TICK_BUDGET=1500/tick: inject(pop) ≈ N/(1500×TPS_avg) — 150k=103s, 450k>608s, 750k может упереться в 1800s cap.
10. POST-workflow-dispatch {} = 204; подтверждение run-id через /actions/runs?branch=<ветка>.
