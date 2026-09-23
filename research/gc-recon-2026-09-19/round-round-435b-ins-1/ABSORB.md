# absorb ROUND (round-435b-ins-1, run 35927377609, branch round-435b-ins-1, head b85a8fc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7154300 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=26, TPS-поллов=6 -> **FAIL**
- T3: median=1.90 @ 7154300 (поллов=5); TPS_exp=2.31; normalized=-17.6%
- GC: young=108, Full=9, total=20.3s, avg=174ms, max=2460ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113934 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.45% (-0.72%) флэт
  - fluid: 16.72% -> 15.70% (-1.02%) спад
  - broadphase: 15.66% -> 13.98% (-1.68%) спад
  - nav_ai: 14.16% -> 13.39% (-0.77%) флэт
  - inside_volatile: 12.01% -> 15.09% (+3.08%) РОСТ
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 7.38% (+0.37%) флэт
  - paletted: 6.41% -> 5.90% (-0.51%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **CRASH-REFUTED**
