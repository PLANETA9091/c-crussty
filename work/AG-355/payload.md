# AG-355 w526 payload — sim64+sim96 sim-миды
- Ветка: swarm-526-355 @2171d6da775975c4bee94748f549ad16f02074e1 (zero-code ref, tree-3296 blobs
  >=3200 anti-sparse гейт PASS, POST 201 13:27Z).
- Dispatch: POST /actions/workflows/bench-v2.yml/dispatches ref=swarm-526-355, 2/2 HTTP 204,
  POST-гэп 31s (канон >=30s), диспатчи только на своей ветке, 2/2 бюджета агента.
- Leg A: run 37013197181 QUEUED 13:28:16Z — sim_distance=64, seed=527355, r1136/9000s/1d/fp4/dgw256/dcp900/xmx10G.
- Leg B: run 37013271696 QUEUED 13:28:57Z — sim_distance=96, seed=528355, остальной вектор = Leg A.
- Пин-детерминизм: 2171d6da = "AG-138 x525: sim-distance plumbing (SIM_DISTANCE env, canon 32)
  + fake_players input" — 12-инпут схема; a9ff088f/e965bd27 = 10-инпут (без sim/fp), туда sim не ходит (урок AG-229/304).
- Контекст очереди: 0 job-starts 08:22-13:21Z @836q (AG-345) — ноги созреют в дренаже 526/527,
  DISP-финал легален (run-id сохранён).
