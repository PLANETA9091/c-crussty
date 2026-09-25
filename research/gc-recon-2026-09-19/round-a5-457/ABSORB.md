# absorb ROUND (a5-457, run 36131617478, branch round-457-anchor-5, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7443979 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7443979 (поллов=5); TPS_exp=2.37; normalized=-11.3%
- GC: young=120, Full=10, total=29.0s, avg=223ms, max=2890ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116168 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.75% (-0.42%) флэт
  - fluid: 16.72% -> 18.28% (+1.57%) РОСТ
  - broadphase: 15.66% -> 15.43% (-0.23%) флэт
  - nav_ai: 14.16% -> 13.50% (-0.67%) флэт
  - inside_volatile: 12.01% -> 12.33% (+0.32%) флэт
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 5.90% (-1.12%) спад
  - paletted: 6.41% -> 6.36% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
