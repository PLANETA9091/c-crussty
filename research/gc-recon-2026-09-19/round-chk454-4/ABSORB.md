# absorb ROUND (chk454-4, run 36097309249, branch round-454b-chunk-4, head 797ae4f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6904589 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6904589 (поллов=5); TPS_exp=2.25; normalized=+11.0%
- GC: young=106, Full=9, total=19.7s, avg=171ms, max=2720ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105397 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.48% (-0.23%) флэт
  - broadphase: 15.66% -> 9.90% (-5.76%) спад
  - nav_ai: 14.16% -> 3.26% (-10.90%) спад
  - inside_volatile: 12.01% -> 16.20% (+4.20%) РОСТ
  - fastutil: 8.54% -> 6.48% (-2.06%) спад
  - java_util: 7.01% -> 8.42% (+1.41%) РОСТ
  - paletted: 6.41% -> 5.37% (-1.03%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
