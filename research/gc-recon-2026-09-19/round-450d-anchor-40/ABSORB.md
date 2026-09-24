# absorb ROUND (450d-anchor-40, run 36063450757, branch round-450-anchor-40, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7031583 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7031583 (поллов=5); TPS_exp=2.28; normalized=-3.5%
- GC: young=117, Full=10, total=24.6s, avg=193ms, max=2387ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117044 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.45% (-1.72%) спад
  - fluid: 16.72% -> 15.90% (-0.81%) флэт
  - broadphase: 15.66% -> 14.38% (-1.28%) спад
  - nav_ai: 14.16% -> 13.81% (-0.35%) флэт
  - inside_volatile: 12.01% -> 11.89% (-0.12%) флэт
  - fastutil: 8.54% -> 8.41% (-0.12%) флэт
  - java_util: 7.01% -> 6.97% (-0.04%) флэт
  - paletted: 6.41% -> 6.20% (-0.21%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
