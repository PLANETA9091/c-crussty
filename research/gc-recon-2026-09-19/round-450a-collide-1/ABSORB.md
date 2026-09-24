# absorb ROUND (450a-collide-1, run 36061445187, branch round-450a-collide, head 3768c55)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7011907 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7011907 (поллов=5); TPS_exp=2.28; normalized=+1.1%
- GC: young=119, Full=9, total=18.9s, avg=148ms, max=2334ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=101252 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.28% (+0.57%) флэт
  - broadphase: 15.66% -> 10.25% (-5.41%) спад
  - nav_ai: 14.16% -> 5.99% (-8.18%) спад
  - inside_volatile: 12.01% -> 12.86% (+0.86%) флэт
  - fastutil: 8.54% -> 5.92% (-2.62%) спад
  - java_util: 7.01% -> 7.79% (+0.78%) флэт
  - paletted: 6.41% -> 6.03% (-0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
