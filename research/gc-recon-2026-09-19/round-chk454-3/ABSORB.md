# absorb ROUND (chk454-3, run 36094585474, branch round-454b-chunk-3, head d6e77f4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6530144 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6530144 (поллов=5); TPS_exp=2.17; normalized=+15.0%
- GC: young=107, Full=9, total=22.3s, avg=192ms, max=3141ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104415 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.92% (+0.20%) флэт
  - broadphase: 15.66% -> 9.59% (-6.06%) спад
  - nav_ai: 14.16% -> 3.24% (-10.92%) спад
  - inside_volatile: 12.01% -> 16.39% (+4.38%) РОСТ
  - fastutil: 8.54% -> 6.88% (-1.66%) спад
  - java_util: 7.01% -> 8.44% (+1.43%) РОСТ
  - paletted: 6.41% -> 5.83% (-0.57%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
