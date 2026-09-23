# absorb ROUND (b2l3, run 35811233192, branch round-422-b-brain2-l3, head 290c214)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6606436 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6606436 (поллов=6); TPS_exp=2.19; normalized=+14.1%
- GC: young=4137, Full=11, total=45.4s, avg=11ms, max=2612ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110491 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.28% (-2.43%) спад
  - broadphase: 15.66% -> 16.02% (+0.36%) флэт
  - nav_ai: 14.16% -> 5.45% (-8.71%) спад
  - inside_volatile: 12.01% -> 11.79% (-0.22%) флэт
  - fastutil: 8.54% -> 8.92% (+0.38%) флэт
  - java_util: 7.01% -> 7.61% (+0.59%) флэт
  - paletted: 6.41% -> 5.42% (-0.98%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
