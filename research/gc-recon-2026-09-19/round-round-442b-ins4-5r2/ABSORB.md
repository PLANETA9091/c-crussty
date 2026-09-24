# absorb ROUND (round-442b-ins4-5r2, run 35960093553, branch round-442b-ins4-5r2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7034189 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7034189 (поллов=5); TPS_exp=2.28; normalized=+5.3%
- GC: young=108, Full=9, total=19.4s, avg=166ms, max=2358ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106969 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.93% (-0.79%) флэт
  - broadphase: 15.66% -> 9.77% (-5.89%) спад
  - nav_ai: 14.16% -> 3.93% (-10.23%) спад
  - inside_volatile: 12.01% -> 17.13% (+5.13%) РОСТ
  - fastutil: 8.54% -> 6.64% (-1.89%) спад
  - java_util: 7.01% -> 8.47% (+1.46%) РОСТ
  - paletted: 6.41% -> 5.35% (-1.06%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
