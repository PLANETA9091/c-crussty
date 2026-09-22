# absorb ROUND (g3b, run 35747884647, branch round-416-b-g3b, head b1dbc66)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9294211 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 9294211 (поллов=5); TPS_exp=2.76; normalized=-5.7%
- GC: young=115, Full=10, total=27.3s, avg=218ms, max=3234ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112822 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.06% (-0.65%) флэт
  - broadphase: 15.66% -> 14.99% (-0.67%) флэт
  - nav_ai: 14.16% -> 6.76% (-7.40%) спад
  - inside_volatile: 12.01% -> 12.49% (+0.49%) флэт
  - fastutil: 8.54% -> 8.46% (-0.07%) флэт
  - java_util: 7.01% -> 7.74% (+0.72%) флэт
  - paletted: 6.41% -> 6.86% (+0.45%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
