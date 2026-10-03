# AG-139 MEMORY (≤15 строк уроков)
1. Cancel in_progress = реально освобождает слот (202); dead-letter AG-101 = только queued-раны.
2. dgw≤128 на 3-dim стенде = G4-FAIL death-march (nether/end=0, 3ч+): нижний край w-кривой не мерить без урезания dims.
3. FALSE-DRAIN канон AG-116 применяется и к валидным SUCCESS-ногам: window < pregen floor → ch/s не S-валид.
4. run_started_at = время создания рана, не старта джобы: смотреть actions/runs/{id}/jobs → started_at.
5. Артефакты download: /actions/artifacts/{id}/zip, −L обязателен; BENCHV2.md + server-stdout.log внутри.
6. «0 success с 14:36Z»-тайминги протухают за часы: перед famine-выводом проверять completed с 17:00Z.
7. WBP-смоуки голодают за bv2-3ч-ногами при 40 ip: слот-модель w528 = длина ног, не «0 ip».
8. Master доска append через contents CAS: payload ≤~600KB ок, но -d "$VAR" ломает ARG_MAX → temp-file.
9. Merge-ревизия перед любым union-кандидатом: master дрейфует по 7+ мёржей за день (107 stale за 5ч).
10. GitHub artifacts живут (expired=False) даже у 12ч-зомби — харвест дешевле re-POST.

# AG-139 w528 (iter-1: w4096-vs-w3072 sameboot re-fire)
11. sameboot lever: leg A = input dim_gen_window, leg B = leg_b_vars="K=V" (экспорт перебивает job-env);
    leg_id УНИКАЛЕН на диспатч — concurrency cancel-in-progress снесёт близнеца.
12. ≤2 POST/агента: min-of-3 = 2 своих POST + prereg-спека хэндоффа 3-й ноги (sb3) в claims/clm.
13. Рекорды n=1 вне band = FALSE-DRAIN/бимодал-suspect до sameboot-репликации; вердикт = prereg-гейты.
