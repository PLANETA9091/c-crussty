# absorb ROUND (round-442-chunk4-5, run 35957129139, branch round-442-chunk4-5, head c5fe025)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8724698 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.95 @ 8724698 (поллов=6); TPS_exp=2.64; normalized=+11.9%
- GC: young=114, Full=8, total=22.1s, avg=181ms, max=2957ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104783 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.35% (+1.63%) РОСТ
  - broadphase: 15.66% -> 9.66% (-5.99%) спад
  - nav_ai: 14.16% -> 3.53% (-10.64%) спад
  - inside_volatile: 12.01% -> 17.01% (+5.00%) РОСТ
  - fastutil: 8.54% -> 6.41% (-2.13%) спад
  - java_util: 7.01% -> 8.84% (+1.83%) РОСТ
  - paletted: 6.41% -> 6.44% (+0.04%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
