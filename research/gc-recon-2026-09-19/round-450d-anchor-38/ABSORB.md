# absorb ROUND (450d-anchor-38, run 36063426544, branch round-450-anchor-38, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8004753 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 8004753 (поллов=5); TPS_exp=2.48; normalized=-11.5%
- GC: young=116, Full=10, total=27.0s, avg=214ms, max=2507ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114166 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.37% (-1.80%) спад
  - fluid: 16.72% -> 16.91% (+0.20%) флэт
  - broadphase: 15.66% -> 14.86% (-0.80%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.45%) флэт
  - inside_volatile: 12.01% -> 10.80% (-1.20%) спад
  - fastutil: 8.54% -> 8.68% (+0.14%) флэт
  - java_util: 7.01% -> 6.70% (-0.32%) флэт
  - paletted: 6.41% -> 7.05% (+0.65%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
