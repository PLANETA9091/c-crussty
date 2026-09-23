# absorb ROUND (round-433-cce-l2r4, run 35901527695, branch round-433-cce-l2r4, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8723195 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8723195 (поллов=6); TPS_exp=2.64; normalized=+21.4%
- GC: young=121, Full=9, total=18.1s, avg=139ms, max=2020ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102402 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.41% (-0.31%) флэт
  - broadphase: 15.66% -> 9.24% (-6.41%) спад
  - nav_ai: 14.16% -> 3.21% (-10.95%) спад
  - inside_volatile: 12.01% -> 15.37% (+3.37%) РОСТ
  - fastutil: 8.54% -> 6.39% (-2.14%) спад
  - java_util: 7.01% -> 8.42% (+1.41%) РОСТ
  - paletted: 6.41% -> 5.95% (-0.46%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
