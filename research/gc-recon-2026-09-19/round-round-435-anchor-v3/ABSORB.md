# absorb ROUND (round-435-anchor-v3, run 35924517029, branch round-435-anchor-v3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6775381 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=2.20 @ 6775381 (поллов=5); TPS_exp=2.23; normalized=-1.2%
- GC: young=114, Full=9, total=20.4s, avg=166ms, max=2357ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117210 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.80% (-1.38%) спад
  - fluid: 16.72% -> 16.28% (-0.44%) флэт
  - broadphase: 15.66% -> 15.12% (-0.53%) флэт
  - nav_ai: 14.16% -> 13.24% (-0.92%) флэт
  - inside_volatile: 12.01% -> 11.66% (-0.35%) флэт
  - fastutil: 8.54% -> 8.65% (+0.11%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.29% (-0.11%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **CRASH-REFUTED**
