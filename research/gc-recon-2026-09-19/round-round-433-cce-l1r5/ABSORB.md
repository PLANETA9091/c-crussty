# absorb ROUND (round-433-cce-l1r5, run 35901472358, branch round-433-cce-l1r5, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7481365 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 7481365 (поллов=5); TPS_exp=2.37; normalized=+22.1%
- GC: young=112, Full=9, total=22.6s, avg=187ms, max=3307ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105319 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.36% (+1.64%) РОСТ
  - broadphase: 15.66% -> 9.96% (-5.69%) спад
  - nav_ai: 14.16% -> 3.63% (-10.53%) спад
  - inside_volatile: 12.01% -> 17.16% (+5.16%) РОСТ
  - fastutil: 8.54% -> 5.72% (-2.82%) спад
  - java_util: 7.01% -> 8.50% (+1.49%) РОСТ
  - paletted: 6.41% -> 5.49% (-0.92%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
