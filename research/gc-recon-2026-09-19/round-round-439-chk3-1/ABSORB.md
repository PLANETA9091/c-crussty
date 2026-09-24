# absorb ROUND (round-439-chk3-1, run 35946074177, branch round-439-chk3-1, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6707079 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6707079 (поллов=5); TPS_exp=2.21; normalized=+13.1%
- GC: young=102, Full=9, total=19.6s, avg=177ms, max=2738ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104288 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.18% (-0.53%) флэт
  - broadphase: 15.66% -> 9.75% (-5.91%) спад
  - nav_ai: 14.16% -> 3.29% (-10.88%) спад
  - inside_volatile: 12.01% -> 15.99% (+3.99%) РОСТ
  - fastutil: 8.54% -> 6.34% (-2.20%) спад
  - java_util: 7.01% -> 8.22% (+1.21%) РОСТ
  - paletted: 6.41% -> 5.43% (-0.98%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
