# absorb ROUND (bqc, run 35764284850, branch round-417-c-bqc, head bad02d8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6837461 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6837461 (поллов=5); TPS_exp=2.24; normalized=+16.1%
- GC: young=1135, Full=9, total=24.4s, avg=21ms, max=1947ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107517 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.37% (-1.34%) спад
  - broadphase: 15.66% -> 14.39% (-1.27%) спад
  - nav_ai: 14.16% -> 9.24% (-4.93%) спад
  - inside_volatile: 12.01% -> 11.96% (-0.05%) флэт
  - fastutil: 8.54% -> 8.32% (-0.22%) флэт
  - java_util: 7.01% -> 7.94% (+0.93%) флэт
  - paletted: 6.41% -> 5.21% (-1.20%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
