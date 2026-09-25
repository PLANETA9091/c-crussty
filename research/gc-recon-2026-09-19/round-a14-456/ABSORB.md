# absorb ROUND (a14-456, run 36112215042, branch round-456-anchor-14, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7202898 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 7202898 (поллов=5); TPS_exp=2.32; normalized=-13.6%
- GC: young=106, Full=9, total=19.7s, avg=172ms, max=2486ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115787 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.76% (-1.41%) спад
  - fluid: 16.72% -> 15.77% (-0.94%) флэт
  - broadphase: 15.66% -> 15.60% (-0.06%) флэт
  - nav_ai: 14.16% -> 14.52% (+0.36%) флэт
  - inside_volatile: 12.01% -> 11.28% (-0.73%) флэт
  - fastutil: 8.54% -> 9.01% (+0.48%) флэт
  - java_util: 7.01% -> 6.51% (-0.51%) флэт
  - paletted: 6.41% -> 6.24% (-0.17%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
