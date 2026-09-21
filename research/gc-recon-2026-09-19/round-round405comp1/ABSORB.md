# absorb ROUND (round405comp1, run 35630922520, branch round-405-comp-l1, head fef3746)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7069582 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7069582 (поллов=5); TPS_exp=2.29; normalized=+18.0%
- GC: young=115, Full=9, total=21.7s, avg=175ms, max=2609ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110458 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.13% (+1.41%) РОСТ
  - broadphase: 15.66% -> 14.85% (-0.80%) флэт
  - nav_ai: 14.16% -> 9.77% (-4.40%) спад
  - inside_volatile: 12.01% -> 13.04% (+1.04%) РОСТ
  - fastutil: 8.54% -> 7.10% (-1.44%) спад
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 5.86% (-0.55%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
