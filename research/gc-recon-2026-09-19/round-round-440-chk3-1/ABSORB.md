# absorb ROUND (round-440-chk3-1, run 35950582686, branch round-440-chk3-1, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8427117 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8427117 (поллов=5); TPS_exp=2.57; normalized=+8.8%
- GC: young=130, Full=10, total=19.4s, avg=139ms, max=2110ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102424 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.53% (-0.18%) флэт
  - broadphase: 15.66% -> 9.46% (-6.20%) спад
  - nav_ai: 14.16% -> 3.09% (-11.07%) спад
  - inside_volatile: 12.01% -> 15.47% (+3.46%) РОСТ
  - fastutil: 8.54% -> 6.45% (-2.09%) спад
  - java_util: 7.01% -> 8.57% (+1.55%) РОСТ
  - paletted: 6.41% -> 5.98% (-0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
