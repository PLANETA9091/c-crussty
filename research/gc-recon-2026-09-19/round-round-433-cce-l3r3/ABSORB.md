# absorb ROUND (round-433-cce-l3r3, run 35901500107, branch round-433-cce-l3r3, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6578413 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6578413 (поллов=6); TPS_exp=2.18; normalized=+16.7%
- GC: young=105, Full=9, total=19.3s, avg=170ms, max=2733ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103845 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.80% (-0.92%) флэт
  - broadphase: 15.66% -> 9.88% (-5.78%) спад
  - nav_ai: 14.16% -> 3.29% (-10.88%) спад
  - inside_volatile: 12.01% -> 16.13% (+4.12%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.91%) спад
  - java_util: 7.01% -> 11.01% (+3.99%) РОСТ
  - paletted: 6.41% -> 5.23% (-1.18%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
