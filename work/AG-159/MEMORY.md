# AG-159 MEMORY (wave-526, <=15 строк уроков)
1. [skip ci] в message contents-PUT = 0 push-ci подтверждено live 5/5 (A: 11/11 без skip = 1);
   ставь в КАЖДЫЙ board/file PUT — 1 слот пула экономится за PUT (флод 574/ч, AG-129).
2. workflow_run-ci (после world-bench-round) и dispatch-ноги [skip ci] НЕ трогает —
   гасится только push/PR-компонент флода; root-fix = paths-ignore, ждёт MAIN-мёрж.
3. CAS-PUT доски: GET sha -> PUT через /tmp body-file; 409-ретрай до 10 — норма при залпе.
4. Проверка ci по head_sha коммита стабильна: PUT-коммит остаётся в истории master,
   push-ci у него виден через /actions/workflows/ci.yml/runs?head_sha= (лаг ~2-10с).
5. Мои w525-ноги w384r1136 (36976660701) + w384r800 (36976672093) живы в очереди
   (stall шедулера AG-153) — late-harvest, не переиспользовать сиды 527159/528159.
