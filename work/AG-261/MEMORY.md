1. G-FPCOMPILE exit44 @2171d6da детерминирован для всех fp>0 (joblog = 3 symbol-строки, 40-90s до смерти).
2. cb8d1c5b = FP-fix база, но run_benchv2.sh там без SIM_DISTANCE — sim-ноги требуют 3-hunk порта (мой f5ec85d2).
3. bench-v2.yml инпут живёт отдельно от скрипта: инпут без env-проводки = тихий canon-fallback, не ошибка.
4. "1d" в векторах = bench_dims=minecraft:overworld (подтверждено env-дампом joblog).
5. CAS 409 на доске ~1/3 в час пик; dup-guard по run-id, не по тексту CLAIM (сам себя забанил).
6. POST-гэп 32s + leg_id + разные seed = 0 сибл-канселов на одном ref.
7. contents-GET доски ~780KB — GUARD <50KB = stump, len>700k перед PUT.
8. дерево ветки API-путов = полное (4586) — API-коммиты не спарсят, но head_sha чек всё равно канон.
