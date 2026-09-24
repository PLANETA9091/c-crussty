# absorb ROUND (451a-ins4-1, run 36069553109, branch round-451-ins4-1, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6745660 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6745660 (поллов=5); TPS_exp=2.22; normalized=+12.6%
- GC: young=103, Full=9, total=21.0s, avg=188ms, max=3161ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107148 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.25% (-0.46%) флэт
  - broadphase: 15.66% -> 9.85% (-5.81%) спад
  - nav_ai: 14.16% -> 3.69% (-10.48%) спад
  - inside_volatile: 12.01% -> 17.13% (+5.12%) РОСТ
  - fastutil: 8.54% -> 6.29% (-2.25%) спад
  - java_util: 7.01% -> 8.44% (+1.42%) РОСТ
  - paletted: 6.41% -> 5.43% (-0.97%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
