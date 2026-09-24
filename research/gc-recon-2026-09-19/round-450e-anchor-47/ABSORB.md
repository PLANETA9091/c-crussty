# absorb ROUND (450e-anchor-47, run 36066541754, branch round-450-anchor-47, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6944070 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6944070 (поллов=5); TPS_exp=2.26; normalized=-2.7%
- GC: young=110, Full=9, total=21.1s, avg=178ms, max=2373ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116860 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.64% (-1.53%) спад
  - fluid: 16.72% -> 15.80% (-0.91%) флэт
  - broadphase: 15.66% -> 15.13% (-0.53%) флэт
  - nav_ai: 14.16% -> 13.87% (-0.29%) флэт
  - inside_volatile: 12.01% -> 11.41% (-0.59%) флэт
  - fastutil: 8.54% -> 9.10% (+0.57%) флэт
  - java_util: 7.01% -> 6.59% (-0.42%) флэт
  - paletted: 6.41% -> 6.32% (-0.09%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
