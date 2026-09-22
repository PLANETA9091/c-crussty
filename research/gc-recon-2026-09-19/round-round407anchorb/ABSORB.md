# absorb ROUND (round407anchorb, run 35659205186, branch round-407-anchorb, head d3e20bc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6976430 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6976430 (поллов=5); TPS_exp=2.27; normalized=-3.0%
- GC: young=111, Full=9, total=21.9s, avg=183ms, max=2899ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115339 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.69% (-0.48%) флэт
  - fluid: 16.72% -> 16.65% (-0.06%) флэт
  - broadphase: 15.66% -> 15.88% (+0.22%) флэт
  - nav_ai: 14.16% -> 14.79% (+0.63%) флэт
  - inside_volatile: 12.01% -> 11.77% (-0.23%) флэт
  - fastutil: 8.54% -> 9.27% (+0.73%) флэт
  - java_util: 7.01% -> 7.09% (+0.08%) флэт
  - paletted: 6.41% -> 6.60% (+0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
