# absorb ROUND (agent-a-430anchor-a, run 35873081453, branch round-430-anchor-a, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7079659 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7079659 (поллов=5); TPS_exp=2.29; normalized=-8.3%
- GC: young=110, Full=9, total=20.4s, avg=171ms, max=2440ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116843 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.26% (-0.91%) флэт
  - fluid: 16.72% -> 16.57% (-0.15%) флэт
  - broadphase: 15.66% -> 15.56% (-0.10%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.67%) флэт
  - inside_volatile: 12.01% -> 11.58% (-0.43%) флэт
  - fastutil: 8.54% -> 8.60% (+0.06%) флэт
  - java_util: 7.01% -> 6.66% (-0.36%) флэт
  - paletted: 6.41% -> 6.02% (-0.39%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
