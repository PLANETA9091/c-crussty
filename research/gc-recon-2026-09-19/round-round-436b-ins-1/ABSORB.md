# absorb ROUND (round-436b-ins-1, run 35932077379, branch round-436b-ins-1, head c5d3985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8625694 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 8625694 (поллов=5); TPS_exp=2.62; normalized=+10.9%
- GC: young=116, Full=9, total=21.7s, avg=174ms, max=2881ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107530 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.78% (+1.07%) РОСТ
  - broadphase: 15.66% -> 9.50% (-6.16%) спад
  - nav_ai: 14.16% -> 3.83% (-10.33%) спад
  - inside_volatile: 12.01% -> 17.90% (+5.90%) РОСТ
  - fastutil: 8.54% -> 6.26% (-2.28%) спад
  - java_util: 7.01% -> 8.58% (+1.56%) РОСТ
  - paletted: 6.41% -> 6.21% (-0.20%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
