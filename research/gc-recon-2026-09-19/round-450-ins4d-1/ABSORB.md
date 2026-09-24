# absorb ROUND (450-ins4d-1, run 36050703263, branch round-450-ins4d-1, head d9d1fb3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6441203 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6441203 (поллов=5); TPS_exp=2.16; normalized=+6.7%
- GC: young=104, Full=9, total=19.0s, avg=168ms, max=2522ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107884 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.26% (-0.45%) флэт
  - broadphase: 15.66% -> 9.78% (-5.87%) спад
  - nav_ai: 14.16% -> 3.81% (-10.36%) спад
  - inside_volatile: 12.01% -> 16.43% (+4.43%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.92%) спад
  - java_util: 7.01% -> 7.92% (+0.91%) флэт
  - paletted: 6.41% -> 5.19% (-1.22%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
