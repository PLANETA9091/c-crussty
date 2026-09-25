# absorb ROUND (poi457-9, run 36134005127, branch round-456b-poi-9, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7054948 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7054948 (поллов=5); TPS_exp=2.28; normalized=+5.1%
- GC: young=104, Full=9, total=19.4s, avg=172ms, max=2483ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103744 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.70% (-1.02%) спад
  - broadphase: 15.66% -> 9.68% (-5.97%) спад
  - nav_ai: 14.16% -> 3.26% (-10.90%) спад
  - inside_volatile: 12.01% -> 16.65% (+4.64%) РОСТ
  - fastutil: 8.54% -> 6.18% (-2.35%) спад
  - java_util: 7.01% -> 8.29% (+1.27%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.07%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
