# absorb ROUND (round-435c-chk-4, run 35925961774, branch round-435c-chk-4, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6704948 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6704948 (поллов=5); TPS_exp=2.21; normalized=+22.1%
- GC: young=109, Full=9, total=20.9s, avg=178ms, max=2758ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104088 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.28% (+0.57%) флэт
  - broadphase: 15.66% -> 9.62% (-6.04%) спад
  - nav_ai: 14.16% -> 3.23% (-10.94%) спад
  - inside_volatile: 12.01% -> 15.54% (+3.54%) РОСТ
  - fastutil: 8.54% -> 6.82% (-1.72%) спад
  - java_util: 7.01% -> 8.44% (+1.43%) РОСТ
  - paletted: 6.41% -> 6.28% (-0.13%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
