# absorb ROUND (poi456-4, run 36126738321, branch round-456b-poi-4, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8957260 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 8957260 (поллов=5); TPS_exp=2.69; normalized=+15.4%
- GC: young=121, Full=10, total=19.8s, avg=151ms, max=2258ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102544 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.50% (-0.21%) флэт
  - broadphase: 15.66% -> 8.93% (-6.73%) спад
  - nav_ai: 14.16% -> 3.23% (-10.94%) спад
  - inside_volatile: 12.01% -> 15.71% (+3.71%) РОСТ
  - fastutil: 8.54% -> 6.28% (-2.26%) спад
  - java_util: 7.01% -> 8.72% (+1.71%) РОСТ
  - paletted: 6.41% -> 5.92% (-0.48%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
