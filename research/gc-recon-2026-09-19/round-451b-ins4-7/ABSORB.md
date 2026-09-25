# absorb ROUND (451b-ins4-7, run 36073717864, branch round-451-ins4-7, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6963727 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6963727 (поллов=5); TPS_exp=2.27; normalized=+19.2%
- GC: young=105, Full=9, total=19.4s, avg=171ms, max=2745ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106822 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.87% (-0.85%) флэт
  - broadphase: 15.66% -> 9.98% (-5.68%) спад
  - nav_ai: 14.16% -> 3.89% (-10.27%) спад
  - inside_volatile: 12.01% -> 17.09% (+5.09%) РОСТ
  - fastutil: 8.54% -> 6.71% (-1.83%) спад
  - java_util: 7.01% -> 8.43% (+1.42%) РОСТ
  - paletted: 6.41% -> 5.21% (-1.19%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
