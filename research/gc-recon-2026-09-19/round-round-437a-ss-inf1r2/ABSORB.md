# absorb ROUND (round-437a-ss-inf1r2, run 35939635762, branch round-437a-ss-inf1r2, head 3f3b111)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7134873 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7134873 (поллов=5); TPS_exp=2.30; normalized=-8.8%
- GC: young=107, Full=10, total=23.0s, avg=196ms, max=2375ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116574 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.63% (-1.54%) спад
  - fluid: 16.72% -> 16.00% (-0.71%) флэт
  - broadphase: 15.66% -> 14.90% (-0.76%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 11.32% (-0.69%) флэт
  - fastutil: 8.54% -> 8.60% (+0.06%) флэт
  - java_util: 7.01% -> 6.86% (-0.16%) флэт
  - paletted: 6.41% -> 6.00% (-0.41%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
