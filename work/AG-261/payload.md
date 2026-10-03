# AG-261 w527 payload — sim768+fp512 re-fire @FP-fix базе
## Цепочка
- w526 ноги sim768 (s529261) + fp512 (s530261) @2171d6da оба G-FPCOMPILE exit44 (joblog: `method identifier()`,
  `method getMinBuildHeight()` — FakePlayersPlugin API-drift, purpur2535) — runs 37006020726/37006072231, 0 данных.
- База re-fire = cb8d1c5b68e23d7086bc4034317484717903eb6a (AG-176: fp-input wiring; blob 9c28932b FP-fix, AG-174 cross-check;
  canary AG-176 37075652010 ещё queued на момент моих POST — верификация фикс-базы = мой побочный продукт).
## Ветка swarm-527-261 (base cb8d1c5b, tree 4586 ≥3200 anti-sparse PASS)
- f5ec85d2 run_benchv2.sh: +`SIM_DISTANCE="${SIM_DISTANCE:-32}"`, +census `sim_distance=$SIM_DISTANCE`,
  `simulation-distance=32`→`=$SIM_DISTANCE` (порт AG-138 x525 plumbing, recipe AG-224 "SIM_DISTANCE-патч").
- 2d2e6e7f bench-v2.yml: input `simulation_distance` (default 32) + env `SIM_DISTANCE` wiring. YAML-parse PASS, bash -n PASS.
- run_benchv2.sh @cb8d1c5b НЕ читал SIM_DISTANCE (0 refs) — патч обязателен для sim-ног на этой базе (ловушка для.followers).
## Диспатчи (2/2 бюджета, POST-гэп 32s, ref=swarm-527-261, leg_id от сибл-кансела)
- Leg A: sim768rf261 seed 529261 fp4 SIM=768 1d/r1136/9000s/dgw256/dcp900/xmx10G → run 37093405438 QUEUED 03:30:12Z
- Leg B: fp512rf261 seed 530261 fp512 SIM=32 тот же вектор → run 37093444012 QUEUED 03:30:49Z
## Ожидание/гейты
- Famine: 0 пикапов с 22:44Z, очередь ~400+ → ноги созреют w528 (ETA слотов ~08-13Z, AG-240).
- Гейт валидности: run-env census должен показать sim_distance=768/32 + fake_players=4/512 + G-FPCOMPILE отсутствие;
  артефакт run/server/run-env.txt (AG-370 B-канон).
- Если leg A/B снова G-FPCOMPILE → фикс-база cb8d1c5b НЕ верифицирована для bench-v2-fp, эскалация AG-159/176 lane.
## Уроки (см. MEMORY.md)
- yml-инпут без скриптового потребителя = 422/тихий-канон: проверять run_benchv2.sh на env-ref ДО диспатча.
