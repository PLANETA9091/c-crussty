# absorb ROUND (round409aleg3, run 35667055638, branch round-405-a-l3, head e88f73b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6021509 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6021509 (поллов=5); TPS_exp=2.07; normalized=-3.2%
- GC: young=109, Full=9, total=23.8s, avg=202ms, max=2908ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112440 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.43% (-0.74%) флэт
  - fluid: 16.72% -> 17.19% (+0.48%) флэт
  - broadphase: 15.66% -> 15.30% (-0.35%) флэт
  - nav_ai: 14.16% -> 13.25% (-0.91%) флэт
  - inside_volatile: 12.01% -> 11.59% (-0.42%) флэт
  - fastutil: 8.54% -> 8.75% (+0.21%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 7.21% (+0.81%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
