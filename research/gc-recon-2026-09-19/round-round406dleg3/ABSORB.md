# absorb ROUND (round406dleg3, run 35655138217, branch round-406-d-l3, head e904b5f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6603401 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6603401 (поллов=6); TPS_exp=2.19; normalized=+14.2%
- GC: young=110, Full=9, total=21.9s, avg=184ms, max=2376ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111451 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.47% (+3.30%) РОСТ
  - fluid: 16.72% -> 18.64% (+1.93%) РОСТ
  - broadphase: 15.66% -> 15.30% (-0.35%) флэт
  - nav_ai: 14.16% -> 8.59% (-5.57%) спад
  - inside_volatile: 12.01% -> 11.13% (-0.87%) флэт
  - fastutil: 8.54% -> 7.22% (-1.31%) спад
  - java_util: 7.01% -> 7.25% (+0.24%) флэт
  - paletted: 6.41% -> 6.90% (+0.50%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
