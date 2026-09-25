# absorb ROUND (chkmono456-1, run 36122381392, branch round-456c-chunkmono-1, head 270fac2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=2924, col=PARALLEL, runner=6879602 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6879602 (поллов=5); TPS_exp=2.25; normalized=+20.1%
- GC: young=112, Full=9, total=21.9s, avg=181ms, max=2835ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104168 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.22% (+1.51%) РОСТ
  - broadphase: 15.66% -> 10.75% (-4.91%) спад
  - nav_ai: 14.16% -> 2.94% (-11.23%) спад
  - inside_volatile: 12.01% -> 17.35% (+5.35%) РОСТ
  - fastutil: 8.54% -> 6.81% (-1.73%) спад
  - java_util: 7.01% -> 9.08% (+2.07%) РОСТ
  - paletted: 6.41% -> 6.23% (-0.17%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **DELIVERY-FAIL**
