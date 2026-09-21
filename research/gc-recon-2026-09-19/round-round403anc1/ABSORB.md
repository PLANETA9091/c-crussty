# absorb ROUND (round403anc1, run 35604150450, branch master, head f19d5f5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6489026 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=2.10 @ 6489026 (поллов=5); TPS_exp=2.17; normalized=-3.0%
- GC: young=112, Full=9, total=23.9s, avg=198ms, max=2698ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112572 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.58% (-0.59%) флэт
  - fluid: 16.72% -> 17.57% (+0.85%) флэт
  - broadphase: 15.66% -> 15.57% (-0.09%) флэт
  - nav_ai: 14.16% -> 14.13% (-0.03%) флэт
  - inside_volatile: 12.01% -> 11.42% (-0.58%) флэт
  - fastutil: 8.54% -> 8.92% (+0.38%) флэт
  - java_util: 7.01% -> 6.96% (-0.05%) флэт
  - paletted: 6.41% -> 7.32% (+0.92%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **CRASH-REFUTED**
