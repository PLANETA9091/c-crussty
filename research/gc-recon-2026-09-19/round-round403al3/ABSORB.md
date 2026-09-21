# absorb ROUND (round403al3, run 35611904072, branch round-403-a-leg3, head 0d88371)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7140519 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7140519 (поллов=6); TPS_exp=2.30; normalized=+17.3%
- GC: young=110, Full=9, total=20.4s, avg=171ms, max=2912ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110468 сэмплов (базлайн 115655)
  - items: 31.17% -> 36.16% (+4.99%) РОСТ
  - fluid: 16.72% -> 19.09% (+2.37%) РОСТ
  - broadphase: 15.66% -> 11.21% (-4.45%) спад
  - nav_ai: 14.16% -> 6.98% (-7.18%) спад
  - inside_volatile: 12.01% -> 12.47% (+0.46%) флэт
  - fastutil: 8.54% -> 7.10% (-1.43%) спад
  - java_util: 7.01% -> 7.72% (+0.71%) флэт
  - paletted: 6.41% -> 6.33% (-0.07%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
