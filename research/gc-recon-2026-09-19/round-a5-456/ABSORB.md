# absorb ROUND (a5-456, run 36112119983, branch round-456-anchor-5, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7054384 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7054384 (поллов=5); TPS_exp=2.28; normalized=-3.7%
- GC: young=111, Full=9, total=20.9s, avg=174ms, max=2408ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116820 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.46% (-1.71%) спад
  - fluid: 16.72% -> 15.63% (-1.09%) спад
  - broadphase: 15.66% -> 15.64% (-0.02%) флэт
  - nav_ai: 14.16% -> 14.16% (-0.00%) флэт
  - inside_volatile: 12.01% -> 11.48% (-0.52%) флэт
  - fastutil: 8.54% -> 8.98% (+0.45%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.37% (-0.04%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
