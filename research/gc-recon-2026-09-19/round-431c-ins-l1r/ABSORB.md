# absorb ROUND (431c-ins-l1r, run 35886964557, branch round-431c-ins-l1r, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6581993 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6581993 (поллов=5); TPS_exp=2.18; normalized=+14.4%
- GC: young=103, Full=9, total=19.5s, avg=174ms, max=2434ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102997 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.35% (-0.36%) флэт
  - broadphase: 15.66% -> 9.74% (-5.91%) спад
  - nav_ai: 14.16% -> 3.26% (-10.90%) спад
  - inside_volatile: 12.01% -> 14.85% (+2.84%) РОСТ
  - fastutil: 8.54% -> 6.68% (-1.86%) спад
  - java_util: 7.01% -> 8.59% (+1.58%) РОСТ
  - paletted: 6.41% -> 5.56% (-0.84%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
