# absorb ROUND (450d-anchor-33, run 36063360661, branch round-450-anchor-33, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7113264 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7113264 (поллов=5); TPS_exp=2.30; normalized=-8.6%
- GC: young=116, Full=9, total=21.5s, avg=172ms, max=2357ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116310 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.25% (-1.92%) спад
  - fluid: 16.72% -> 16.16% (-0.56%) флэт
  - broadphase: 15.66% -> 15.49% (-0.16%) флэт
  - nav_ai: 14.16% -> 14.38% (+0.22%) флэт
  - inside_volatile: 12.01% -> 11.68% (-0.33%) флэт
  - fastutil: 8.54% -> 9.00% (+0.46%) флэт
  - java_util: 7.01% -> 6.37% (-0.65%) флэт
  - paletted: 6.41% -> 6.01% (-0.39%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
