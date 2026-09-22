# absorb ROUND (anchorc3, run 35762940608, branch round-417-anchorc3, head 87fc0cb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6938112 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6938112 (поллов=5); TPS_exp=2.26; normalized=-7.1%
- GC: young=108, Full=9, total=20.6s, avg=176ms, max=2483ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115323 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.53% (-0.64%) флэт
  - fluid: 16.72% -> 16.64% (-0.07%) флэт
  - broadphase: 15.66% -> 16.70% (+1.04%) РОСТ
  - nav_ai: 14.16% -> 14.57% (+0.41%) флэт
  - inside_volatile: 12.01% -> 11.54% (-0.47%) флэт
  - fastutil: 8.54% -> 9.24% (+0.70%) флэт
  - java_util: 7.01% -> 6.84% (-0.17%) флэт
  - paletted: 6.41% -> 6.20% (-0.20%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
