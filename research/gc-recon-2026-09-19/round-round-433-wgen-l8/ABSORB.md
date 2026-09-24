# absorb ROUND (round-433-wgen-l8, run 35901583704, branch round-433-wgen-l8, head 898650c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6823307 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 6823307 (поллов=6); TPS_exp=2.24; normalized=+23.0%
- GC: young=107, Full=9, total=20.0s, avg=172ms, max=2618ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102686 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.76% (+0.04%) флэт
  - broadphase: 15.66% -> 10.11% (-5.55%) спад
  - nav_ai: 14.16% -> 3.50% (-10.66%) спад
  - inside_volatile: 12.01% -> 12.79% (+0.78%) флэт
  - fastutil: 8.54% -> 6.93% (-1.61%) спад
  - java_util: 7.01% -> 7.82% (+0.81%) флэт
  - paletted: 6.41% -> 5.63% (-0.77%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
