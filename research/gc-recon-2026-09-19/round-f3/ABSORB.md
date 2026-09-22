# absorb ROUND (f3, run 35753053201, branch round-416-c-f3, head 8e47eb0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6737497 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6737497 (поллов=5); TPS_exp=2.22; normalized=+17.2%
- GC: young=1130, Full=9, total=24.8s, avg=22ms, max=2063ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111491 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.52% (+1.81%) РОСТ
  - broadphase: 15.66% -> 13.37% (-2.28%) спад
  - nav_ai: 14.16% -> 9.12% (-5.04%) спад
  - inside_volatile: 12.01% -> 12.13% (+0.12%) флэт
  - fastutil: 8.54% -> 7.51% (-1.03%) спад
  - java_util: 7.01% -> 7.50% (+0.48%) флэт
  - paletted: 6.41% -> 5.23% (-1.18%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
