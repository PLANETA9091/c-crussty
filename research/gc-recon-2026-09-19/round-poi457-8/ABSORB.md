# absorb ROUND (poi457-8, run 36132385298, branch round-456b-poi-8, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=10425941 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.40 @ 10425941 (поллов=5); TPS_exp=2.99; normalized=+13.5%
- GC: young=123, Full=9, total=19.9s, avg=150ms, max=2300ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104540 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.72% (+0.00%) флэт
  - broadphase: 15.66% -> 9.24% (-6.42%) спад
  - nav_ai: 14.16% -> 3.69% (-10.47%) спад
  - inside_volatile: 12.01% -> 17.33% (+5.32%) РОСТ
  - fastutil: 8.54% -> 6.50% (-2.04%) спад
  - java_util: 7.01% -> 9.04% (+2.03%) РОСТ
  - paletted: 6.41% -> 6.58% (+0.17%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
