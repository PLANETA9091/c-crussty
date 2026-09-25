# absorb ROUND (a3-456, run 36112098614, branch round-456-anchor-3, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7123081 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 7123081 (поллов=5); TPS_exp=2.30; normalized=-13.0%
- GC: young=108, Full=10, total=23.4s, avg=198ms, max=2415ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116738 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.20% (-1.97%) спад
  - fluid: 16.72% -> 15.40% (-1.32%) спад
  - broadphase: 15.66% -> 14.60% (-1.06%) спад
  - nav_ai: 14.16% -> 14.29% (+0.13%) флэт
  - inside_volatile: 12.01% -> 10.96% (-1.04%) спад
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.31% (-0.71%) флэт
  - paletted: 6.41% -> 6.25% (-0.15%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
