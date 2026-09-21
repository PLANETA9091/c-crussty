# absorb ROUND (round403sc2a, run 35615072803, branch round-403-sc2a, head d93894b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6701449 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6701449 (поллов=6); TPS_exp=2.21; normalized=+13.1%
- GC: young=107, Full=9, total=18.9s, avg=163ms, max=2308ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112534 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.29% (+0.58%) флэт
  - broadphase: 15.66% -> 13.80% (-1.86%) спад
  - nav_ai: 14.16% -> 11.28% (-2.88%) спад
  - inside_volatile: 12.01% -> 12.51% (+0.51%) флэт
  - fastutil: 8.54% -> 8.01% (-0.53%) флэт
  - java_util: 7.01% -> 7.13% (+0.12%) флэт
  - paletted: 6.41% -> 6.36% (-0.04%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
