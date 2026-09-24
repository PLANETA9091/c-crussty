# absorb ROUND (round-442b-ins4d-2, run 35958918146, branch round-442b-ins4d-2, head b67768f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8641617 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8641617 (поллов=5); TPS_exp=2.62; normalized=+22.2%
- GC: young=126, Full=9, total=17.1s, avg=126ms, max=2011ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103596 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.66% (-0.06%) флэт
  - broadphase: 15.66% -> 8.96% (-6.69%) спад
  - nav_ai: 14.16% -> 3.96% (-10.21%) спад
  - inside_volatile: 12.01% -> 15.75% (+3.74%) РОСТ
  - fastutil: 8.54% -> 6.03% (-2.51%) спад
  - java_util: 7.01% -> 7.96% (+0.95%) флэт
  - paletted: 6.41% -> 5.69% (-0.72%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
