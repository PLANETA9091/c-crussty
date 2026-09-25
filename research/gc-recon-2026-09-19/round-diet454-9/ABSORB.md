# absorb ROUND (diet454-9, run 36098974370, branch round-454c-diet-9, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6355330 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6355330 (поллов=5); TPS_exp=2.14; normalized=+12.3%
- GC: young=100, Full=8, total=19.0s, avg=176ms, max=2592ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106052 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.08% (-0.63%) флэт
  - broadphase: 15.66% -> 9.81% (-5.84%) спад
  - nav_ai: 14.16% -> 3.21% (-10.95%) спад
  - inside_volatile: 12.01% -> 16.66% (+4.65%) РОСТ
  - fastutil: 8.54% -> 6.12% (-2.41%) спад
  - java_util: 7.01% -> 8.65% (+1.63%) РОСТ
  - paletted: 6.41% -> 5.14% (-1.27%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
