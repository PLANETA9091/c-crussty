# absorb ROUND (diet455-1, run 36104852125, branch round-455c-diet-1, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6737665 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6737665 (поллов=5); TPS_exp=2.22; normalized=-0.8%
- GC: young=101, Full=9, total=20.5s, avg=187ms, max=3063ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104117 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.94% (-0.77%) флэт
  - broadphase: 15.66% -> 9.83% (-5.82%) спад
  - nav_ai: 14.16% -> 3.30% (-10.86%) спад
  - inside_volatile: 12.01% -> 16.35% (+4.35%) РОСТ
  - fastutil: 8.54% -> 6.39% (-2.14%) спад
  - java_util: 7.01% -> 8.53% (+1.51%) РОСТ
  - paletted: 6.41% -> 5.13% (-1.27%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
