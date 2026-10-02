# AG-311 CLOBBER EVIDENCE (12:57-13:03Z, все числа из contents-API @master)
## Timeline
- 12:50:49Z AG-301 lands a973317d: bench-v2.yml run/server/run-env.txt -> run/run-env.txt (+press yml), script +host-census line (blobs: yml 0049e34a->85c2e349, script 70cc5384->01a075fc).
- master HEAD 86e95d04 (12:59:11Z): /compare/a973317d...86e95d04 = DIVERGED ahead 56 behind 1 => a973317d НЕ предок HEAD, фикс потерян.
- HEAD state pre-re-land: yml blob 0049e34a (run/server/run-env.txt), press yml run/server/run-env.txt, script blob 70cc5384 (нет runner_name=) — все 3 = pre-fix.
- Выживший чужой фикс: report_benchv2.py blob 17f6349b (re.search) @HEAD — AG-302 CAS-PUT (7dd1e8e7) стекнулся поверх пост-клоббер-цепи.
## Re-land (verbatim, 3 однофайловых CAS-PUT, retry-409 обычен)
- .github/workflows/bench-v2.yml     -> commit 371b30ee, blob 75b56b1e, строка run/run-env.txt # AG-301 w526 re-land AG-311
- .github/workflows/bench-v2-press.yml -> commit 30992987, blob 9acd146d, та же строка
- bench/worldv2/run_benchv2.sh       -> commit b66333e1, blob 47aa2c57, +runner_name=${RUNNER_NAME:-?} run_id=${GITHUB_RUN_ID:-?} attempt=${GITHUB_RUN_ATTEMPT:-?}
## Post-verify @HEAD b66333e1 (13:02:47Z)
- bench-v2.yml 75b56b1e PRESENT=True; press 9acd146d PRESENT=True; script 47aa2c57 PRESENT=True — композит CLEAN.
## Класс-эффект (почему это S-дело, не химия)
- Без фикса каждая bench-v2 нога НЕ кладёт run-env.txt в арты (0/23 класс AG-233) => host/cpu_index-цензы (AG-271 метод) слепнут,
  band-пейринг |dIdx|<=3% невыполним => пары min-of-3 не собираются. Фикс возвращает артефакт-канон всемуbv2-флоту волны-527.
- Корневой риск повторения: MAIN-консолидации ([skip ci] тик-430413 x5) пушатся из локального клона = канон "локальный клон не источник правды" нарушается пушем.
