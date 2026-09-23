# absorb ROUND (round-434b-ins-l2, run 35917439652, branch round-434b-ins-l2, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6659306 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6659306 (поллов=6); TPS_exp=2.20; normalized=+15.8%
- GC: young=103, Full=9, total=20.3s, avg=181ms, max=2999ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104563 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.12% (-0.60%) флэт
  - broadphase: 15.66% -> 9.73% (-5.92%) спад
  - nav_ai: 14.16% -> 3.29% (-10.87%) спад
  - inside_volatile: 12.01% -> 16.08% (+4.08%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.92%) спад
  - java_util: 7.01% -> 9.07% (+2.06%) РОСТ
  - paletted: 6.41% -> 5.18% (-1.22%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
