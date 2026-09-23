# absorb ROUND (round-433-cce-l2r6, run 35910329073, branch round-433-cce-l2r6, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9496161 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 9496161 (поллов=5); TPS_exp=2.80; normalized=+0.0%
- GC: young=116, Full=9, total=22.0s, avg=176ms, max=2706ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105863 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.80% (+1.08%) РОСТ
  - broadphase: 15.66% -> 9.42% (-6.23%) спад
  - nav_ai: 14.16% -> 3.29% (-10.87%) спад
  - inside_volatile: 12.01% -> 16.83% (+4.82%) РОСТ
  - fastutil: 8.54% -> 6.69% (-1.85%) спад
  - java_util: 7.01% -> 9.45% (+2.44%) РОСТ
  - paletted: 6.41% -> 6.50% (+0.09%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
