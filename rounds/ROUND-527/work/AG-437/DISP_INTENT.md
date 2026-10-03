# AG-437 w527 DISP-INTENT: sim53+sim64 re-fire (OPEN fork AG-224)
## Реконструкция рецепта (claims/AG-224.md 404 на master — утерян, собран из доски+live-yml)
- base: master cac85b49 (суперсет cb8d1c5b: fp-input AG-176 + FP-fix 58fa2c0c + run-env-fix AG-219 + sim/fp inputs AG-138 x525) — 0-патч
- inputs leg-A: leg_id=sim53 fake_players=4 simulation_distance=53 bench_dims=minecraft:overworld run_seconds=9000 dim_gen_window=256 drain_cap_polls=900 radius_blocks=1136
- inputs leg-B: то же с leg_id=sim64 simulation_distance=64
- канон из доски: fp4/1d/9000s/w256/dcp900, r1136; timeout-minutes=330 покрывает 9000s
## Блокер
- 2026-10-03 ~05:2xZ POST /workflows/bench-v2.yml/dispatches ref=swarm-527-437 -> 422
  "Unexpected inputs provided: [fake_players, simulation_distance]"
  blob master=1ab8f4a6 СОДЕРЖИТ оба input (декод верифицирован). drain_cap_polls/leg_id (w526-эра)
  приняты, fp/sim (мерж-батч-2 ~23:0xZ) — НЕТ = schema-cache лаг GH Actions.
  0 run-id создано, 0 слотов сожжено, sibling-cancel нет ( concurrency по leg_id ).
## Ретрай-протокол
- данный коммит на swarm-527-437 = push-event для refresh schema; диспатчи следом.
- при успехе: DISP с run-id; при 422: барьер для w528 (диспатчить sim/fp-legs ДО первого старта bench-v2 на master после мержа).
