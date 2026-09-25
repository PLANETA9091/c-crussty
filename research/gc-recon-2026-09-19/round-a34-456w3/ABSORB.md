# absorb ROUND (a34-456w3, run 36119699687, branch round-456-anchor-34, head d4deb33)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6520174 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6520174 (поллов=5); TPS_exp=2.17; normalized=+5.9%
- GC: young=112, Full=10, total=27.0s, avg=221ms, max=2596ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115781 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.07% (-2.11%) спад
  - fluid: 16.72% -> 16.29% (-0.43%) флэт
  - broadphase: 15.66% -> 14.90% (-0.76%) флэт
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 10.68% (-1.32%) спад
  - fastutil: 8.54% -> 8.43% (-0.10%) флэт
  - java_util: 7.01% -> 6.47% (-0.54%) флэт
  - paletted: 6.41% -> 6.83% (+0.42%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
