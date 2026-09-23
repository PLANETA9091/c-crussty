# absorb ROUND (b2l1, run 35811223073, branch round-422-b-brain2-l1, head 290c214)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6682135 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6682135 (поллов=5); TPS_exp=2.21; normalized=+13.3%
- GC: young=4134, Full=10, total=45.4s, avg=11ms, max=2456ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110854 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.57% (-2.14%) спад
  - broadphase: 15.66% -> 15.00% (-0.66%) флэт
  - nav_ai: 14.16% -> 5.60% (-8.57%) спад
  - inside_volatile: 12.01% -> 12.07% (+0.07%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 7.54% (+0.53%) флэт
  - paletted: 6.41% -> 5.40% (-1.01%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
