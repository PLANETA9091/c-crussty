# absorb ROUND (round-437-chk3-1, run 35935315920, branch round-437-chk3-1, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6807160 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 6807160 (поллов=6); TPS_exp=2.23; normalized=+18.7%
- GC: young=105, Full=9, total=19.3s, avg=169ms, max=2444ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104644 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.41% (-0.31%) флэт
  - broadphase: 15.66% -> 9.93% (-5.73%) спад
  - nav_ai: 14.16% -> 3.25% (-10.92%) спад
  - inside_volatile: 12.01% -> 14.94% (+2.93%) РОСТ
  - fastutil: 8.54% -> 6.29% (-2.25%) спад
  - java_util: 7.01% -> 8.14% (+1.13%) РОСТ
  - paletted: 6.41% -> 5.54% (-0.87%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
