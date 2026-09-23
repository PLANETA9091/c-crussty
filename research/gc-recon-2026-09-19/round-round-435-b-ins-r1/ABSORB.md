# absorb ROUND (round-435-b-ins-r1, run 35924383597, branch round-435-b-ins-r1, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6955628 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6955628 (поллов=6); TPS_exp=2.26; normalized=+8.2%
- GC: young=100, Full=9, total=19.8s, avg=182ms, max=3097ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104019 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.18% (-0.53%) флэт
  - broadphase: 15.66% -> 9.97% (-5.68%) спад
  - nav_ai: 14.16% -> 3.17% (-10.99%) спад
  - inside_volatile: 12.01% -> 16.60% (+4.60%) РОСТ
  - fastutil: 8.54% -> 7.03% (-1.50%) спад
  - java_util: 7.01% -> 8.47% (+1.46%) РОСТ
  - paletted: 6.41% -> 5.18% (-1.22%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
