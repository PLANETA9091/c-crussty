# absorb ROUND (round-443g-ins4d-2, run 36038568089, branch round-443g-ins4d-2, head d9d1fb3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6745000 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6745000 (поллов=5); TPS_exp=2.22; normalized=+12.6%
- GC: young=103, Full=9, total=20.5s, avg=183ms, max=3032ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107281 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.53% (-0.18%) флэт
  - broadphase: 15.66% -> 9.71% (-5.94%) спад
  - nav_ai: 14.16% -> 3.85% (-10.31%) спад
  - inside_volatile: 12.01% -> 17.08% (+5.07%) РОСТ
  - fastutil: 8.54% -> 6.40% (-2.14%) спад
  - java_util: 7.01% -> 8.69% (+1.68%) РОСТ
  - paletted: 6.41% -> 5.44% (-0.97%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
