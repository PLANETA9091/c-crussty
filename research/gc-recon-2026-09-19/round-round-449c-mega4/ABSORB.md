# absorb ROUND (round-449c-mega4, run 36026595604, branch round-449c-mega4, head 5188f18)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8945613 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.95 @ 8945613 (поллов=6); TPS_exp=2.68; normalized=+10.0%
- GC: young=119, Full=10, total=20.1s, avg=155ms, max=2297ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102230 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.34% (-0.37%) флэт
  - broadphase: 15.66% -> 9.09% (-6.56%) спад
  - nav_ai: 14.16% -> 3.14% (-11.03%) спад
  - inside_volatile: 12.01% -> 15.72% (+3.71%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.21%) спад
  - java_util: 7.01% -> 8.77% (+1.75%) РОСТ
  - paletted: 6.41% -> 5.61% (-0.80%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
