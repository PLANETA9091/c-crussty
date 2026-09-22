# absorb ROUND (f2, run 35753034236, branch round-416-c-f2, head 8e47eb0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=5719483 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 5719483 (поллов=5); TPS_exp=2.00; normalized=+24.8%
- GC: young=1132, Full=9, total=25.9s, avg=23ms, max=2329ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111320 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.51% (+1.80%) РОСТ
  - broadphase: 15.66% -> 14.13% (-1.53%) спад
  - nav_ai: 14.16% -> 9.36% (-4.80%) спад
  - inside_volatile: 12.01% -> 11.65% (-0.36%) флэт
  - fastutil: 8.54% -> 7.50% (-1.04%) спад
  - java_util: 7.01% -> 7.77% (+0.75%) флэт
  - paletted: 6.41% -> 5.22% (-1.18%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
