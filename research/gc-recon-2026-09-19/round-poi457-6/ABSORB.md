# absorb ROUND (poi457-6, run 36131797497, branch round-456b-poi-6, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6997574 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6997574 (поллов=6); TPS_exp=2.27; normalized=+12.2%
- GC: young=110, Full=9, total=20.1s, avg=169ms, max=2717ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106112 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.27% (-0.44%) флэт
  - broadphase: 15.66% -> 9.56% (-6.10%) спад
  - nav_ai: 14.16% -> 3.12% (-11.05%) спад
  - inside_volatile: 12.01% -> 16.98% (+4.98%) РОСТ
  - fastutil: 8.54% -> 6.88% (-1.65%) спад
  - java_util: 7.01% -> 8.30% (+1.28%) РОСТ
  - paletted: 6.41% -> 5.26% (-1.14%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
