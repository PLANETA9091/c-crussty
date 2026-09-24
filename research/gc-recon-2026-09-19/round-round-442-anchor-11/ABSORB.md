# absorb ROUND (round-442-anchor-11, run 35957153836, branch round-442-anchor-11, head 335f170)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6890823 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6890823 (поллов=5); TPS_exp=2.25; normalized=-6.7%
- GC: young=109, Full=9, total=21.0s, avg=178ms, max=2468ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116944 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.76% (-1.42%) спад
  - fluid: 16.72% -> 15.68% (-1.04%) спад
  - broadphase: 15.66% -> 15.24% (-0.42%) флэт
  - nav_ai: 14.16% -> 13.93% (-0.23%) флэт
  - inside_volatile: 12.01% -> 11.37% (-0.63%) флэт
  - fastutil: 8.54% -> 8.88% (+0.34%) флэт
  - java_util: 7.01% -> 6.54% (-0.47%) флэт
  - paletted: 6.41% -> 6.04% (-0.37%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
