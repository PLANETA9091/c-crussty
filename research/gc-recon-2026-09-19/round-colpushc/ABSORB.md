# absorb ROUND (colpushc, run 35781347034, branch round-419-a-cpc, head dc4afd9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=3714, col=PARALLEL, runner=6430808 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.55 @ 6430808 (поллов=6); TPS_exp=2.15; normalized=-28.0%
- GC: young=1057, Full=10, total=65.7s, avg=62ms, max=2104ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114224 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 12.29% (-4.43%) спад
  - broadphase: 15.66% -> 10.54% (-5.12%) спад
  - nav_ai: 14.16% -> 17.92% (+3.76%) РОСТ
  - inside_volatile: 12.01% -> 8.75% (-3.25%) спад
  - fastutil: 8.54% -> 6.30% (-2.24%) спад
  - java_util: 7.01% -> 5.88% (-1.14%) спад
  - paletted: 6.41% -> 4.41% (-2.00%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
