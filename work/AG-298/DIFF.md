# AG-298 diff (swarm-526-298 @4d29dd0cbb vs master @8364de7c)
.github/workflows/bench-v2.yml, Upload artifacts (блок path), 1 строка:
-            run/server/run-env.txt
+            run/run-env.txt
Обоснование: run_benchv2.sh:14 WORK=${BENCH_WORK:-$PWD/run}; :17 cd "$WORK/server";
:38 cat > "$WORK/run-env.txt" (=> run/run-env.txt); :177 append туда же.
report_benchv2.py:13-16: _envp = dirname(server_dir)/"run-env.txt" — канон родителя.
Эффект: артефакт benchv2-ag433 получает run-env.txt (radius/seed/dims/fake_players/
runner_cpu_index/purpur-хеши/mark_mode) → host-census ch/s (σ_run атрибуция, AG-233)
восстанавливается для всех будущих benchv2-ног.
