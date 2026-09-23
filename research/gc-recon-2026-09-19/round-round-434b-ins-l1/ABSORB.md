# absorb ROUND (round-434b-ins-l1, run 35917427142, branch round-434b-ins-l1, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6991563 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6991563 (поллов=5); TPS_exp=2.27; normalized=+1.3%
- GC: young=107, Full=9, total=19.3s, avg=166ms, max=2422ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104121 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.67% (-1.05%) спад
  - broadphase: 15.66% -> 9.77% (-5.88%) спад
  - nav_ai: 14.16% -> 3.28% (-10.88%) спад
  - inside_volatile: 12.01% -> 16.35% (+4.35%) РОСТ
  - fastutil: 8.54% -> 6.88% (-1.66%) спад
  - java_util: 7.01% -> 8.70% (+1.69%) РОСТ
  - paletted: 6.41% -> 5.20% (-1.21%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
