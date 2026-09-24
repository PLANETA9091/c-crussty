# absorb ROUND (450-anchor-10, run 36050621515, branch round-450-anchor-10, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6966488 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6966488 (поллов=5); TPS_exp=2.27; normalized=-7.3%
- GC: young=105, Full=9, total=20.3s, avg=178ms, max=2407ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116323 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.60% (-1.57%) спад
  - fluid: 16.72% -> 15.68% (-1.03%) спад
  - broadphase: 15.66% -> 16.00% (+0.34%) флэт
  - nav_ai: 14.16% -> 14.10% (-0.06%) флэт
  - inside_volatile: 12.01% -> 11.13% (-0.87%) флэт
  - fastutil: 8.54% -> 8.50% (-0.04%) флэт
  - java_util: 7.01% -> 7.10% (+0.09%) флэт
  - paletted: 6.41% -> 6.07% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
