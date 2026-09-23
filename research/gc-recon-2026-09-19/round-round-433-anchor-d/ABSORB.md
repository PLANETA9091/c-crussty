# absorb ROUND (round-433-anchor-d, run 35901513048, branch round-433-anchor-d, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6126695 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6126695 (поллов=6); TPS_exp=2.09; normalized=+10.1%
- GC: young=111, Full=9, total=23.9s, avg=199ms, max=3072ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115640 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.20% (-1.97%) спад
  - fluid: 16.72% -> 16.59% (-0.13%) флэт
  - broadphase: 15.66% -> 15.28% (-0.37%) флэт
  - nav_ai: 14.16% -> 13.66% (-0.50%) флэт
  - inside_volatile: 12.01% -> 10.73% (-1.27%) спад
  - fastutil: 8.54% -> 8.53% (-0.00%) флэт
  - java_util: 7.01% -> 6.57% (-0.44%) флэт
  - paletted: 6.41% -> 7.03% (+0.63%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
