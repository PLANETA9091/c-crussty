# absorb ROUND (450c-chunk-3r1, run 36054213318, branch round-450c-chunk-3r1, head 8302586)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7141084 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7141084 (поллов=6); TPS_exp=2.30; normalized=+8.6%
- GC: young=109, Full=9, total=20.6s, avg=174ms, max=2858ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104586 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.32% (-0.39%) флэт
  - broadphase: 15.66% -> 9.90% (-5.76%) спад
  - nav_ai: 14.16% -> 3.16% (-11.00%) спад
  - inside_volatile: 12.01% -> 16.20% (+4.19%) РОСТ
  - fastutil: 8.54% -> 6.56% (-1.98%) спад
  - java_util: 7.01% -> 8.41% (+1.40%) РОСТ
  - paletted: 6.41% -> 5.16% (-1.25%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
