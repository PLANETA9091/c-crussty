# absorb ROUND (round-439-chk-1, run 35946090438, branch round-439-chk-1, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7239578 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 7239578 (поллов=6); TPS_exp=2.32; normalized=+9.7%
- GC: young=108, Full=9, total=20.2s, avg=172ms, max=2918ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104025 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.56% (-0.15%) флэт
  - broadphase: 15.66% -> 9.76% (-5.89%) спад
  - nav_ai: 14.16% -> 3.25% (-10.92%) спад
  - inside_volatile: 12.01% -> 16.76% (+4.75%) РОСТ
  - fastutil: 8.54% -> 6.68% (-1.86%) спад
  - java_util: 7.01% -> 8.64% (+1.63%) РОСТ
  - paletted: 6.41% -> 5.44% (-0.96%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
