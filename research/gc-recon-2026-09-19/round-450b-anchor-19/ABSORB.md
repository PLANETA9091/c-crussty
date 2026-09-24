# absorb ROUND (450b-anchor-19, run 36055367872, branch round-450-anchor-19, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6880913 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6880913 (поллов=5); TPS_exp=2.25; normalized=-2.1%
- GC: young=105, Full=8, total=20.8s, avg=184ms, max=2438ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116549 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.36% (-1.81%) спад
  - fluid: 16.72% -> 15.76% (-0.95%) флэт
  - broadphase: 15.66% -> 15.79% (+0.14%) флэт
  - nav_ai: 14.16% -> 13.98% (-0.18%) флэт
  - inside_volatile: 12.01% -> 11.21% (-0.79%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 6.77% (-0.24%) флэт
  - paletted: 6.41% -> 6.15% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
