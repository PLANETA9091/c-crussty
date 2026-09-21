# absorb ROUND (round405comp2, run 35630952157, branch round-405-comp-l2, head fef3746)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7214503 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 7214503 (поллов=5); TPS_exp=2.32; normalized=+38.0%
- GC: young=115, Full=9, total=21.7s, avg=175ms, max=2666ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109551 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.45% (+1.74%) РОСТ
  - broadphase: 15.66% -> 14.81% (-0.85%) флэт
  - nav_ai: 14.16% -> 10.13% (-4.04%) спад
  - inside_volatile: 12.01% -> 13.30% (+1.30%) РОСТ
  - fastutil: 8.54% -> 7.05% (-1.49%) спад
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.01% (-0.40%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
