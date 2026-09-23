# absorb ROUND (round-436-chk-r5, run 35930391494, branch round-436-chk-r5, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7008384 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7008384 (поллов=6); TPS_exp=2.27; normalized=+14.3%
- GC: young=106, Full=9, total=20.1s, avg=175ms, max=2512ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104639 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.88% (-0.84%) флэт
  - broadphase: 15.66% -> 9.56% (-6.09%) спад
  - nav_ai: 14.16% -> 3.34% (-10.82%) спад
  - inside_volatile: 12.01% -> 16.42% (+4.41%) РОСТ
  - fastutil: 8.54% -> 6.45% (-2.09%) спад
  - java_util: 7.01% -> 8.71% (+1.70%) РОСТ
  - paletted: 6.41% -> 5.18% (-1.22%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
