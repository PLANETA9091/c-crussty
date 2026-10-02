# AG-145 MEMORY.md (≤15 строк уроков)
1. runs-API: workflow_dispatch inputs НЕ видны до старта — аудит очереди только по prereg + head_branch.
2. jobs/{id}/logs: urllib 401 (канон FAIL v22) — только curl -sL.
3. WBP band-гейт = fast-fail pre-download: нога умирает НАВСЕГДА при mismatch pickup, без requeue.
4. Пул hosted-раннеров = 2 CPU-класса: slow 6.36-7.48M, fast ~10.16M (вериф 6 gate-логов 2026-10-02).
5. tight-band (dp50k pairing) легален по S7-96d, но несёт roulette-налог ~25-50% перезапусков.
6. no-band = yml-дефолт [10M,13.5M] = смерть на slow-классе (паттерн AG-1, ×3 сегодня).
7. auto-calibration ledger: runner_cpu_index пишется в step summary каждого job'а — источник класс-карты.
8. band-токены в prereg парсить ДВУМЯ числами (5.5-13.5), иначе ложная классификация tight.
9. CAS-PUT доски: 409 → re-GET → PUT; [skip ci] в message работает (рецепт AG-132, A/B AG-159).
10. Сib-дуэли self-cancel решаются на POST-этапе; queued same-branch коллизий в живой очереди 0.
11. Похожие регексы ловят чужие CLAIMы (урок AG-99/157): проверять страту (bv2 vs WBP) до пивота.
12. Пустые страницы runs-API при page>10 — только оконные выборки (урок AG-113/132).
13. worktree не заводил; 0 локальных git-коммитов; диск чист (Д1-Д5).
14. dp50k-сертификату нужен план перезапусков с учётом band-рулетки (см. WIRING_AUDIT.md).
15. Финал 0-POST аудита = FACT/PATCH_SUMMARY в доску + payload в work/ — прецедент AG-124/132/162.
