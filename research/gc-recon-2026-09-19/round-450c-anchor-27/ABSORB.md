# absorb ROUND (450c-anchor-27, run 36058468931, branch round-450-anchor-27, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8606390 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8606390 (поллов=5); TPS_exp=2.61; normalized=+3.4%
- GC: young=125, Full=10, total=22.1s, avg=163ms, max=2084ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112725 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.63% (-2.54%) спад
  - fluid: 16.72% -> 16.24% (-0.48%) флэт
  - broadphase: 15.66% -> 14.79% (-0.87%) флэт
  - nav_ai: 14.16% -> 13.62% (-0.54%) флэт
  - inside_volatile: 12.01% -> 10.88% (-1.13%) спад
  - fastutil: 8.54% -> 8.28% (-0.26%) флэт
  - java_util: 7.01% -> 6.79% (-0.23%) флэт
  - paletted: 6.41% -> 6.89% (+0.48%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
