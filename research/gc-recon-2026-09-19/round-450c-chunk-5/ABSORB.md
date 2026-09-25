# absorb ROUND (450c-chunk-5, run 36060569157, branch round-450c-chunk-5, head 201d9d6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7113409 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 7113409 (поллов=6); TPS_exp=2.30; normalized=+11.0%
- GC: young=110, Full=9, total=19.8s, avg=166ms, max=2388ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105284 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.84% (+0.13%) флэт
  - broadphase: 15.66% -> 9.72% (-5.93%) спад
  - nav_ai: 14.16% -> 3.26% (-10.90%) спад
  - inside_volatile: 12.01% -> 16.38% (+4.38%) РОСТ
  - fastutil: 8.54% -> 6.51% (-2.03%) спад
  - java_util: 7.01% -> 8.41% (+1.40%) РОСТ
  - paletted: 6.41% -> 5.43% (-0.97%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
