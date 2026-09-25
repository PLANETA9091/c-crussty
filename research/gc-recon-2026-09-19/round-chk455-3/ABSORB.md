# absorb ROUND (chk455-3, run 36107282243, branch round-455b-chunk-3, head c551f7f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6676696 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6676696 (поллов=5); TPS_exp=2.20; normalized=+8.8%
- GC: young=104, Full=9, total=19.5s, avg=173ms, max=2658ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104838 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 9.70% (-5.96%) спад
  - nav_ai: 14.16% -> 3.24% (-10.92%) спад
  - inside_volatile: 12.01% -> 16.51% (+4.51%) РОСТ
  - fastutil: 8.54% -> 6.59% (-1.95%) спад
  - java_util: 7.01% -> 8.54% (+1.53%) РОСТ
  - paletted: 6.41% -> 5.19% (-1.22%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
