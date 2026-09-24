# absorb ROUND (round-443g-chunk4-2, run 36038610500, branch round-443g-chunk4-2, head c5fe025)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6965346 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6965346 (поллов=5); TPS_exp=2.27; normalized=+5.9%
- GC: young=104, Full=9, total=19.4s, avg=172ms, max=2503ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105139 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.84% (-0.87%) флэт
  - broadphase: 15.66% -> 9.75% (-5.91%) спад
  - nav_ai: 14.16% -> 3.23% (-10.93%) спад
  - inside_volatile: 12.01% -> 15.49% (+3.49%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.21%) спад
  - java_util: 7.01% -> 7.94% (+0.93%) флэт
  - paletted: 6.41% -> 5.12% (-1.29%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
