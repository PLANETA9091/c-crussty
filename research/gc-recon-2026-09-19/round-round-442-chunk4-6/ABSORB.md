# absorb ROUND (round-442-chunk4-6, run 35957177845, branch round-442-chunk4-6, head c5fe025)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7050044 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7050044 (поллов=6); TPS_exp=2.28; normalized=+13.9%
- GC: young=106, Full=9, total=19.8s, avg=172ms, max=2524ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103532 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.23% (-0.48%) флэт
  - broadphase: 15.66% -> 9.79% (-5.87%) спад
  - nav_ai: 14.16% -> 3.28% (-10.89%) спад
  - inside_volatile: 12.01% -> 15.80% (+3.79%) РОСТ
  - fastutil: 8.54% -> 6.78% (-1.76%) спад
  - java_util: 7.01% -> 8.28% (+1.26%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.06%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
