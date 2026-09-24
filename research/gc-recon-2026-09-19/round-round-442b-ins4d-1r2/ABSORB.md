# absorb ROUND (round-442b-ins4d-1r2, run 35960110338, branch round-442b-ins4d-1r2, head d9d1fb3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7126512 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7126512 (поллов=5); TPS_exp=2.30; normalized=+8.7%
- GC: young=103, Full=8, total=17.3s, avg=156ms, max=2318ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107350 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.42% (-0.30%) флэт
  - broadphase: 15.66% -> 9.80% (-5.86%) спад
  - nav_ai: 14.16% -> 3.81% (-10.35%) спад
  - inside_volatile: 12.01% -> 16.73% (+4.73%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.91%) спад
  - java_util: 7.01% -> 8.70% (+1.69%) РОСТ
  - paletted: 6.41% -> 5.17% (-1.23%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
