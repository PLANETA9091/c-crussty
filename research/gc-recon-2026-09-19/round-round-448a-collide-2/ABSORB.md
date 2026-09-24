# absorb ROUND (round-448a-collide-2, run 36018221322, branch round-448a-collide-2, head 56142b5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=4, col=PARALLEL, runner=7038312 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 7038312 (поллов=5); TPS_exp=2.28; normalized=-12.3%
- GC: young=529, Full=9, total=41.6s, avg=77ms, max=2161ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111631 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 13.81% (-2.91%) спад
  - broadphase: 15.66% -> 12.65% (-3.01%) спад
  - nav_ai: 14.16% -> 13.35% (-0.81%) флэт
  - inside_volatile: 12.01% -> 10.74% (-1.27%) спад
  - fastutil: 8.54% -> 6.96% (-1.58%) спад
  - java_util: 7.01% -> 6.89% (-0.12%) флэт
  - paletted: 6.41% -> 4.77% (-1.64%) спад
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **DELIVERY-FAIL**
