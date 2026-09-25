# absorb ROUND (a19-457, run 36135730972, branch round-457-anchor-19, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6815620 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6815620 (поллов=5); TPS_exp=2.23; normalized=-6.0%
- GC: young=111, Full=10, total=23.8s, avg=196ms, max=2451ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115342 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.69% (-1.49%) спад
  - fluid: 16.72% -> 16.35% (-0.37%) флэт
  - broadphase: 15.66% -> 15.69% (+0.03%) флэт
  - nav_ai: 14.16% -> 14.02% (-0.14%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.58%) флэт
  - fastutil: 8.54% -> 8.50% (-0.04%) флэт
  - java_util: 7.01% -> 6.41% (-0.60%) флэт
  - paletted: 6.41% -> 6.36% (-0.04%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
