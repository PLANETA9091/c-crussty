# absorb ROUND (round-439-pd-1, run 35946058457, branch round-439-pd-1, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7075266 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7075266 (поллов=6); TPS_exp=2.29; normalized=+18.0%
- GC: young=118, Full=9, total=24.9s, avg=196ms, max=3300ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117539 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.54% (-0.63%) флэт
  - fluid: 16.72% -> 18.35% (+1.63%) РОСТ
  - broadphase: 15.66% -> 15.25% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.47% (-0.69%) флэт
  - inside_volatile: 12.01% -> 12.07% (+0.06%) флэт
  - fastutil: 8.54% -> 8.38% (-0.15%) флэт
  - java_util: 7.01% -> 6.08% (-0.93%) флэт
  - paletted: 6.41% -> 7.55% (+1.15%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
