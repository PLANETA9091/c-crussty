# absorb ROUND (round-440-ins4-3, run 35950635077, branch round-440-ins4-3, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7097077 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7097077 (поллов=5); TPS_exp=2.29; normalized=+4.6%
- GC: young=101, Full=9, total=19.4s, avg=177ms, max=2548ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106254 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.31% (-0.40%) флэт
  - broadphase: 15.66% -> 10.03% (-5.63%) спад
  - nav_ai: 14.16% -> 3.73% (-10.43%) спад
  - inside_volatile: 12.01% -> 16.67% (+4.66%) РОСТ
  - fastutil: 8.54% -> 6.29% (-2.24%) спад
  - java_util: 7.01% -> 8.75% (+1.73%) РОСТ
  - paletted: 6.41% -> 5.20% (-1.21%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
