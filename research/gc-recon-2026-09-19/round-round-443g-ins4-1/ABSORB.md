# absorb ROUND (round-443g-ins4-1, run 36038486355, branch round-443g-ins4-1, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6728589 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 6728589 (поллов=6); TPS_exp=2.22; normalized=+19.6%
- GC: young=104, Full=9, total=19.4s, avg=172ms, max=2486ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107978 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.15% (-0.57%) флэт
  - broadphase: 15.66% -> 9.58% (-6.08%) спад
  - nav_ai: 14.16% -> 3.57% (-10.60%) спад
  - inside_volatile: 12.01% -> 16.80% (+4.80%) РОСТ
  - fastutil: 8.54% -> 6.27% (-2.26%) спад
  - java_util: 7.01% -> 8.06% (+1.04%) РОСТ
  - paletted: 6.41% -> 5.07% (-1.34%) спад
  - players_packets: 0.01% -> 0.02% (+0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
