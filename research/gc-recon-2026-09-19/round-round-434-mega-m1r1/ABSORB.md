# absorb ROUND (round-434-mega-m1r1, run 35914673028, branch round-434-mega-m1r1, head b15f329)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8427292 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8427292 (поллов=5); TPS_exp=2.57; normalized=+4.9%
- GC: young=114, Full=9, total=22.8s, avg=185ms, max=2902ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105785 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.55% (+1.83%) РОСТ
  - broadphase: 15.66% -> 10.09% (-5.56%) спад
  - nav_ai: 14.16% -> 3.59% (-10.58%) спад
  - inside_volatile: 12.01% -> 12.96% (+0.96%) флэт
  - fastutil: 8.54% -> 6.36% (-2.17%) спад
  - java_util: 7.01% -> 7.83% (+0.81%) флэт
  - paletted: 6.41% -> 6.81% (+0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
