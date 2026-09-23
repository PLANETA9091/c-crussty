# absorb ROUND (421-anchorc, run 35800931828, branch round-421-anchorc, head 2d23f45)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6742251 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6742251 (поллов=6); TPS_exp=2.22; normalized=+14.9%
- GC: young=114, Full=10, total=26.3s, avg=212ms, max=2591ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115737 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.39% (-1.78%) спад
  - fluid: 16.72% -> 16.38% (-0.33%) флэт
  - broadphase: 15.66% -> 14.64% (-1.02%) спад
  - nav_ai: 14.16% -> 13.79% (-0.38%) флэт
  - inside_volatile: 12.01% -> 10.77% (-1.24%) спад
  - fastutil: 8.54% -> 8.75% (+0.21%) флэт
  - java_util: 7.01% -> 6.42% (-0.60%) флэт
  - paletted: 6.41% -> 6.82% (+0.41%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
