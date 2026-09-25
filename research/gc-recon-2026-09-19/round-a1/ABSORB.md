# absorb ROUND (a1, run 36104708419, branch round-455-anchor-1, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6811539 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 6811539 (поллов=5); TPS_exp=2.23; normalized=-14.9%
- GC: young=107, Full=10, total=23.6s, avg=202ms, max=2521ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116357 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.46% (-1.72%) спад
  - fluid: 16.72% -> 15.39% (-1.33%) спад
  - broadphase: 15.66% -> 15.57% (-0.09%) флэт
  - nav_ai: 14.16% -> 14.38% (+0.22%) флэт
  - inside_volatile: 12.01% -> 10.97% (-1.04%) спад
  - fastutil: 8.54% -> 8.61% (+0.08%) флэт
  - java_util: 7.01% -> 6.53% (-0.48%) флэт
  - paletted: 6.41% -> 6.03% (-0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
