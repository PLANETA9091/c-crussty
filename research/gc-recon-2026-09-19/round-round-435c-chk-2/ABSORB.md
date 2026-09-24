# absorb ROUND (round-435c-chk-2, run 35924801032, branch round-435c-chk-2, head a17cde0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6699453 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6699453 (поллов=5); TPS_exp=2.21; normalized=+17.7%
- GC: young=105, Full=9, total=19.4s, avg=170ms, max=2485ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104405 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.21% (-0.50%) флэт
  - broadphase: 15.66% -> 10.02% (-5.64%) спад
  - nav_ai: 14.16% -> 3.28% (-10.89%) спад
  - inside_volatile: 12.01% -> 15.61% (+3.60%) РОСТ
  - fastutil: 8.54% -> 7.02% (-1.52%) спад
  - java_util: 7.01% -> 8.87% (+1.86%) РОСТ
  - paletted: 6.41% -> 5.21% (-1.19%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
