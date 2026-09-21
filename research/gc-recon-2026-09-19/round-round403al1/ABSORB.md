# absorb ROUND (round403al1, run 35606538923, branch round-403-a-leg1, head c8bfbe0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6738182 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6738182 (поллов=5); TPS_exp=2.22; normalized=+8.2%
- GC: young=116, Full=9, total=20.8s, avg=166ms, max=2337ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113870 сэмплов (базлайн 115655)
  - items: 31.17% -> 32.82% (+1.65%) РОСТ
  - fluid: 16.72% -> 17.52% (+0.80%) флэт
  - broadphase: 15.66% -> 13.39% (-2.27%) спад
  - nav_ai: 14.16% -> 11.25% (-2.92%) спад
  - inside_volatile: 12.01% -> 12.54% (+0.54%) флэт
  - fastutil: 8.54% -> 7.96% (-0.58%) флэт
  - java_util: 7.01% -> 7.21% (+0.20%) флэт
  - paletted: 6.41% -> 6.24% (-0.16%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
