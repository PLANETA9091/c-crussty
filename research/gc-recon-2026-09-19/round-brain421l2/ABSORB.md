# absorb ROUND (brain421l2, run 35804778002, branch round-421-a-brain-l2, head 3e8c431)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6815461 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6815461 (поллов=6); TPS_exp=2.23; normalized=+11.9%
- GC: young=1129, Full=9, total=26.7s, avg=23ms, max=2361ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108190 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.31% (-0.41%) флэт
  - broadphase: 15.66% -> 14.40% (-1.26%) спад
  - nav_ai: 14.16% -> 9.25% (-4.92%) спад
  - inside_volatile: 12.01% -> 12.22% (+0.22%) флэт
  - fastutil: 8.54% -> 8.05% (-0.49%) флэт
  - java_util: 7.01% -> 7.94% (+0.92%) флэт
  - paletted: 6.41% -> 5.53% (-0.88%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
