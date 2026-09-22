# absorb ROUND (b2p2r, run 35728745209, branch round-414-a-b2p2r, head 67d0b0e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=10026408 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 10026408 (поллов=5); TPS_exp=2.91; normalized=-7.2%
- GC: young=119, Full=10, total=24.7s, avg=192ms, max=2601ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115974 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.79% (-2.38%) спад
  - fluid: 16.72% -> 17.19% (+0.48%) флэт
  - broadphase: 15.66% -> 14.29% (-1.36%) спад
  - nav_ai: 14.16% -> 13.76% (-0.40%) флэт
  - inside_volatile: 12.01% -> 11.58% (-0.43%) флэт
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.65% (-0.37%) флэт
  - paletted: 6.41% -> 7.35% (+0.94%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
