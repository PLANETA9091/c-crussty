# absorb ROUND (round-441-ins4-4, run 35954918766, branch round-441-ins4-4, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6855477 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6855477 (поллов=6); TPS_exp=2.24; normalized=+9.2%
- GC: young=106, Full=9, total=19.5s, avg=169ms, max=2548ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106779 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.58% (-0.13%) флэт
  - broadphase: 15.66% -> 9.73% (-5.93%) спад
  - nav_ai: 14.16% -> 3.82% (-10.34%) спад
  - inside_volatile: 12.01% -> 16.75% (+4.74%) РОСТ
  - fastutil: 8.54% -> 6.28% (-2.26%) спад
  - java_util: 7.01% -> 8.56% (+1.55%) РОСТ
  - paletted: 6.41% -> 5.50% (-0.91%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
