# absorb ROUND (round-438-ins4-1, run 35941728426, branch round-438-ins4-1, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6919866 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6919866 (поллов=6); TPS_exp=2.26; normalized=+15.2%
- GC: young=99, Full=9, total=19.4s, avg=179ms, max=2795ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107154 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.32% (-0.39%) флэт
  - broadphase: 15.66% -> 10.04% (-5.62%) спад
  - nav_ai: 14.16% -> 3.63% (-10.54%) спад
  - inside_volatile: 12.01% -> 16.55% (+4.54%) РОСТ
  - fastutil: 8.54% -> 6.91% (-1.63%) спад
  - java_util: 7.01% -> 8.71% (+1.70%) РОСТ
  - paletted: 6.41% -> 5.14% (-1.27%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
