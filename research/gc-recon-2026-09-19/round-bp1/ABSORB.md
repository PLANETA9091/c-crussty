# absorb ROUND (bp1, run 35723440147, branch round-413-bp1, head 125c474)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6989196 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.00 @ 6989196 (поллов=6); TPS_exp=2.27; normalized=-56.0%
- GC: young=65, Full=8, total=12.4s, avg=170ms, max=2715ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114295 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 10.91% (-5.80%) спад
  - broadphase: 15.66% -> 6.62% (-9.04%) спад
  - nav_ai: 14.16% -> 2.78% (-11.38%) спад
  - inside_volatile: 12.01% -> 8.41% (-3.60%) спад
  - fastutil: 8.54% -> 4.30% (-4.24%) спад
  - java_util: 7.01% -> 5.66% (-1.35%) спад
  - paletted: 6.41% -> 3.69% (-2.71%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
