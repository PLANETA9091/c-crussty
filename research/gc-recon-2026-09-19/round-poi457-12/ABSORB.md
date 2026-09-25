# absorb ROUND (poi457-12, run 36146408002, branch round-456b-poi-12, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9830285 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.40 @ 9830285 (поллов=5); TPS_exp=2.87; normalized=+18.5%
- GC: young=130, Full=10, total=24.1s, avg=172ms, max=2509ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105120 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.10% (+0.39%) флэт
  - broadphase: 15.66% -> 9.19% (-6.47%) спад
  - nav_ai: 14.16% -> 3.45% (-10.71%) спад
  - inside_volatile: 12.01% -> 17.53% (+5.52%) РОСТ
  - fastutil: 8.54% -> 6.37% (-2.17%) спад
  - java_util: 7.01% -> 8.61% (+1.60%) РОСТ
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
