# absorb ROUND (450-ins4-2, run 36050677953, branch round-450-ins4-2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7023654 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 7023654 (поллов=6); TPS_exp=2.28; normalized=+22.9%
- GC: young=103, Full=9, total=19.4s, avg=173ms, max=3018ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107259 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.45% (-0.26%) флэт
  - broadphase: 15.66% -> 9.80% (-5.86%) спад
  - nav_ai: 14.16% -> 3.71% (-10.45%) спад
  - inside_volatile: 12.01% -> 16.80% (+4.80%) РОСТ
  - fastutil: 8.54% -> 6.87% (-1.67%) спад
  - java_util: 7.01% -> 8.38% (+1.37%) РОСТ
  - paletted: 6.41% -> 5.38% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
