# absorb ROUND (round-428-anchore, run 35856049575, branch round-428-anchore, head dc704c9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6861061 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6861061 (поллов=6); TPS_exp=2.24; normalized=+7.0%
- GC: young=113, Full=10, total=24.1s, avg=196ms, max=2368ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116740 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.85% (-1.32%) спад
  - fluid: 16.72% -> 15.82% (-0.89%) флэт
  - broadphase: 15.66% -> 14.93% (-0.73%) флэт
  - nav_ai: 14.16% -> 13.72% (-0.44%) флэт
  - inside_volatile: 12.01% -> 11.57% (-0.43%) флэт
  - fastutil: 8.54% -> 8.46% (-0.08%) флэт
  - java_util: 7.01% -> 6.63% (-0.38%) флэт
  - paletted: 6.41% -> 6.22% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
