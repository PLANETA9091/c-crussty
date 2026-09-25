# absorb ROUND (a28, run 36108733180, branch round-455-anchor-28, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6190791 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6190791 (поллов=5); TPS_exp=2.10; normalized=+9.4%
- GC: young=115, Full=10, total=26.0s, avg=208ms, max=2625ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115206 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.21% (-0.96%) флэт
  - fluid: 16.72% -> 16.89% (+0.18%) флэт
  - broadphase: 15.66% -> 15.09% (-0.57%) флэт
  - nav_ai: 14.16% -> 13.50% (-0.67%) флэт
  - inside_volatile: 12.01% -> 11.50% (-0.50%) флэт
  - fastutil: 8.54% -> 9.35% (+0.81%) флэт
  - java_util: 7.01% -> 6.70% (-0.31%) флэт
  - paletted: 6.41% -> 6.76% (+0.36%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
