# absorb ROUND (round-433-wgen-l7, run 35901555470, branch round-433-wgen-l7, head 898650c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6732457 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6732457 (поллов=6); TPS_exp=2.22; normalized=+21.8%
- GC: young=104, Full=9, total=21.7s, avg=192ms, max=3005ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102463 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.44% (+0.73%) флэт
  - broadphase: 15.66% -> 9.90% (-5.76%) спад
  - nav_ai: 14.16% -> 3.53% (-10.63%) спад
  - inside_volatile: 12.01% -> 12.25% (+0.24%) флэт
  - fastutil: 8.54% -> 7.06% (-1.48%) спад
  - java_util: 7.01% -> 7.87% (+0.86%) флэт
  - paletted: 6.41% -> 6.40% (-0.01%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
