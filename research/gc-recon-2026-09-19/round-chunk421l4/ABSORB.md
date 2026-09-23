# absorb ROUND (chunk421l4, run 35805623547, branch round-421-c-chunk-l4, head 600e3cc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6816986 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.65 @ 6816986 (поллов=6); TPS_exp=2.23; normalized=-26.2%
- GC: young=98, Full=9, total=23.2s, avg=217ms, max=2720ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115808 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.02% (-2.15%) спад
  - fluid: 16.72% -> 15.17% (-1.55%) спад
  - broadphase: 15.66% -> 15.66% (+0.00%) флэт
  - nav_ai: 14.16% -> 13.69% (-0.47%) флэт
  - inside_volatile: 12.01% -> 10.61% (-1.39%) спад
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 6.53% (-0.48%) флэт
  - paletted: 6.41% -> 5.99% (-0.41%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
