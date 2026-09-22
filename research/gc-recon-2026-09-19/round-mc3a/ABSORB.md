# absorb ROUND (mc3a, run 35765638407, branch round-417-a-mc3a, head 20c9fdc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6787526 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6787526 (поллов=6); TPS_exp=2.23; normalized=+12.2%
- GC: young=107, Full=9, total=18.5s, avg=160ms, max=2249ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106941 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.03% (-0.69%) флэт
  - broadphase: 15.66% -> 9.62% (-6.04%) спад
  - nav_ai: 14.16% -> 4.15% (-10.01%) спад
  - inside_volatile: 12.01% -> 12.21% (+0.21%) флэт
  - fastutil: 8.54% -> 5.86% (-2.68%) спад
  - java_util: 7.01% -> 8.04% (+1.02%) РОСТ
  - paletted: 6.41% -> 5.36% (-1.05%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
