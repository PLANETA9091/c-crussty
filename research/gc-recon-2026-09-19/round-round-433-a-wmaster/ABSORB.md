# absorb ROUND (round-433-a-wmaster, run 35903488559, branch round-433-a-wmaster, head 5a85a6b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6895634 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6895634 (поллов=5); TPS_exp=2.25; normalized=+6.6%
- GC: young=103, Full=9, total=19.9s, avg=177ms, max=2635ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102688 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.93% (+0.22%) флэт
  - broadphase: 15.66% -> 10.71% (-4.95%) спад
  - nav_ai: 14.16% -> 3.35% (-10.81%) спад
  - inside_volatile: 12.01% -> 12.61% (+0.61%) флэт
  - fastutil: 8.54% -> 6.67% (-1.87%) спад
  - java_util: 7.01% -> 7.52% (+0.51%) флэт
  - paletted: 6.41% -> 5.56% (-0.85%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
