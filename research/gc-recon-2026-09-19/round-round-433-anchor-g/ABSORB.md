# absorb ROUND (round-433-anchor-g, run 35907577624, branch round-433-anchor-g, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6577961 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6577961 (поллов=5); TPS_exp=2.18; normalized=+5.3%
- GC: young=111, Full=10, total=26.7s, avg=221ms, max=3093ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116248 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.72% (-1.45%) спад
  - fluid: 16.72% -> 16.65% (-0.07%) флэт
  - broadphase: 15.66% -> 14.92% (-0.73%) флэт
  - nav_ai: 14.16% -> 13.58% (-0.58%) флэт
  - inside_volatile: 12.01% -> 11.05% (-0.96%) флэт
  - fastutil: 8.54% -> 8.36% (-0.18%) флэт
  - java_util: 7.01% -> 6.63% (-0.39%) флэт
  - paletted: 6.41% -> 6.91% (+0.51%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
