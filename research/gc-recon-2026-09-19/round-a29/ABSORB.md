# absorb ROUND (a29, run 36108744317, branch round-455-anchor-29, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6948160 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6948160 (поллов=5); TPS_exp=2.26; normalized=-2.7%
- GC: young=114, Full=9, total=21.1s, avg=171ms, max=2350ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=118625 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.54% (-1.63%) спад
  - fluid: 16.72% -> 15.97% (-0.75%) флэт
  - broadphase: 15.66% -> 15.30% (-0.36%) флэт
  - nav_ai: 14.16% -> 14.16% (+0.00%) флэт
  - inside_volatile: 12.01% -> 11.87% (-0.14%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.83% (-0.18%) флэт
  - paletted: 6.41% -> 6.27% (-0.14%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
