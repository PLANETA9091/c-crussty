# absorb ROUND (round405anchord, run 35631493642, branch round-405-anchord, head 0eb844c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6845027 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6845027 (поллов=5); TPS_exp=2.24; normalized=-6.3%
- GC: young=106, Full=9, total=20.6s, avg=179ms, max=2494ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115605 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.27% (-0.91%) флэт
  - fluid: 16.72% -> 16.24% (-0.48%) флэт
  - broadphase: 15.66% -> 15.35% (-0.31%) флэт
  - nav_ai: 14.16% -> 14.31% (+0.14%) флэт
  - inside_volatile: 12.01% -> 11.57% (-0.44%) флэт
  - fastutil: 8.54% -> 8.98% (+0.44%) флэт
  - java_util: 7.01% -> 7.13% (+0.12%) флэт
  - paletted: 6.41% -> 6.21% (-0.20%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
