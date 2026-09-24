# absorb ROUND (round-439-ins4-2, run 35946081704, branch round-439-ins4-2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7017937 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7017937 (поллов=5); TPS_exp=2.28; normalized=+14.2%
- GC: young=101, Full=9, total=19.3s, avg=175ms, max=2438ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107122 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.58% (-0.13%) флэт
  - broadphase: 15.66% -> 10.18% (-5.48%) спад
  - nav_ai: 14.16% -> 3.64% (-10.52%) спад
  - inside_volatile: 12.01% -> 17.02% (+5.01%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.92%) спад
  - java_util: 7.01% -> 8.92% (+1.90%) РОСТ
  - paletted: 6.41% -> 5.29% (-1.11%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
