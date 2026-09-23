# absorb ROUND (round-433-cce-l2r5, run 35907643775, branch round-433-cce-l2r5, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7209832 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7209832 (поллов=5); TPS_exp=2.32; normalized=+16.5%
- GC: young=113, Full=9, total=23.9s, avg=196ms, max=3454ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105370 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.82% (+2.10%) РОСТ
  - broadphase: 15.66% -> 9.87% (-5.79%) спад
  - nav_ai: 14.16% -> 3.43% (-10.73%) спад
  - inside_volatile: 12.01% -> 17.56% (+5.56%) РОСТ
  - fastutil: 8.54% -> 6.36% (-2.18%) спад
  - java_util: 7.01% -> 8.79% (+1.78%) РОСТ
  - paletted: 6.41% -> 5.39% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
