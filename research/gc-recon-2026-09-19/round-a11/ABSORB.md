# absorb ROUND (a11, run 36104797524, branch round-455-anchor-11, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7161506 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7161506 (поллов=5); TPS_exp=2.31; normalized=-9.0%
- GC: young=108, Full=9, total=20.5s, avg=175ms, max=2381ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116455 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.58% (-1.59%) спад
  - fluid: 16.72% -> 15.75% (-0.97%) флэт
  - broadphase: 15.66% -> 15.68% (+0.02%) флэт
  - nav_ai: 14.16% -> 13.94% (-0.22%) флэт
  - inside_volatile: 12.01% -> 11.66% (-0.35%) флэт
  - fastutil: 8.54% -> 8.87% (+0.33%) флэт
  - java_util: 7.01% -> 6.53% (-0.48%) флэт
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
