# absorb ROUND (mc4b, run 35788915147, branch round-420-mc4b, head 20c9fdc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7087198 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7087198 (поллов=5); TPS_exp=2.29; normalized=-4.0%
- GC: young=102, Full=9, total=18.5s, avg=167ms, max=2230ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106605 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.97% (-0.75%) флэт
  - broadphase: 15.66% -> 9.77% (-5.89%) спад
  - nav_ai: 14.16% -> 4.13% (-10.04%) спад
  - inside_volatile: 12.01% -> 11.66% (-0.34%) флэт
  - fastutil: 8.54% -> 5.60% (-2.94%) спад
  - java_util: 7.01% -> 7.86% (+0.84%) флэт
  - paletted: 6.41% -> 5.40% (-1.01%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
