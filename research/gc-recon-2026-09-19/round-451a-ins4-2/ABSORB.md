# absorb ROUND (451a-ins4-2, run 36069563406, branch round-451-ins4-2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7099728 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7099728 (поллов=5); TPS_exp=2.29; normalized=+13.3%
- GC: young=108, Full=9, total=20.2s, avg=173ms, max=2779ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107853 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.07% (-0.64%) флэт
  - broadphase: 15.66% -> 9.67% (-5.98%) спад
  - nav_ai: 14.16% -> 3.80% (-10.36%) спад
  - inside_volatile: 12.01% -> 16.91% (+4.91%) РОСТ
  - fastutil: 8.54% -> 6.36% (-2.18%) спад
  - java_util: 7.01% -> 8.29% (+1.28%) РОСТ
  - paletted: 6.41% -> 5.31% (-1.09%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
