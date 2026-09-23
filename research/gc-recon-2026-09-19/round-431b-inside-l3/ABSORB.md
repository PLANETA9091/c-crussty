# absorb ROUND (431b-inside-l3, run 35883978409, branch round-431b-inside-l3, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6452904 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6452904 (поллов=5); TPS_exp=2.16; normalized=+15.9%
- GC: young=102, Full=9, total=19.3s, avg=174ms, max=2796ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103746 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.64% (-0.08%) флэт
  - broadphase: 15.66% -> 9.86% (-5.79%) спад
  - nav_ai: 14.16% -> 3.20% (-10.96%) спад
  - inside_volatile: 12.01% -> 14.96% (+2.95%) РОСТ
  - fastutil: 8.54% -> 6.69% (-1.85%) спад
  - java_util: 7.01% -> 8.29% (+1.28%) РОСТ
  - paletted: 6.41% -> 5.52% (-0.88%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
