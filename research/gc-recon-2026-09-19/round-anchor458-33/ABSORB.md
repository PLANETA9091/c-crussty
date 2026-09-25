# absorb ROUND (anchor458-33, run 36146434580, branch round-458-anchor-33, head 96cc270)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6973621 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6973621 (поллов=5); TPS_exp=2.27; normalized=-11.8%
- GC: young=106, Full=9, total=25.2s, avg=219ms, max=2537ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115503 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.05% (-2.12%) спад
  - fluid: 16.72% -> 16.35% (-0.36%) флэт
  - broadphase: 15.66% -> 14.68% (-0.98%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.49%) флэт
  - inside_volatile: 12.01% -> 10.47% (-1.54%) спад
  - fastutil: 8.54% -> 9.12% (+0.58%) флэт
  - java_util: 7.01% -> 6.66% (-0.35%) флэт
  - paletted: 6.41% -> 7.01% (+0.61%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
