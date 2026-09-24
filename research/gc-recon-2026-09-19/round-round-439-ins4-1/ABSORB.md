# absorb ROUND (round-439-ins4-1, run 35946041509, branch round-439-ins4-1, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6681403 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6681403 (поллов=5); TPS_exp=2.21; normalized=+17.9%
- GC: young=105, Full=9, total=19.9s, avg=174ms, max=2561ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107490 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.28% (-0.43%) флэт
  - broadphase: 15.66% -> 10.18% (-5.48%) спад
  - nav_ai: 14.16% -> 3.82% (-10.34%) спад
  - inside_volatile: 12.01% -> 16.71% (+4.70%) РОСТ
  - fastutil: 8.54% -> 6.55% (-1.99%) спад
  - java_util: 7.01% -> 8.22% (+1.20%) РОСТ
  - paletted: 6.41% -> 5.33% (-1.08%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
