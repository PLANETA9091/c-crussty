# absorb ROUND (round-443g-chk3-1, run 36038716727, branch round-443g-chk3-1, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6739220 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6739220 (поллов=6); TPS_exp=2.22; normalized=+17.2%
- GC: young=104, Full=9, total=18.6s, avg=164ms, max=2494ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104831 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.88% (-0.84%) флэт
  - broadphase: 15.66% -> 9.49% (-6.16%) спад
  - nav_ai: 14.16% -> 3.22% (-10.94%) спад
  - inside_volatile: 12.01% -> 16.05% (+4.04%) РОСТ
  - fastutil: 8.54% -> 6.44% (-2.10%) спад
  - java_util: 7.01% -> 8.55% (+1.54%) РОСТ
  - paletted: 6.41% -> 5.13% (-1.28%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
