# absorb ROUND (round-449-anchor-1, run 36043732389, branch round-449-anchor-1, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7134246 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.05 @ 7134246 (поллов=6); TPS_exp=2.30; normalized=-10.9%
- GC: young=102, Full=9, total=20.1s, avg=181ms, max=2520ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116111 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.62% (-1.55%) спад
  - fluid: 16.72% -> 15.92% (-0.79%) флэт
  - broadphase: 15.66% -> 15.33% (-0.33%) флэт
  - nav_ai: 14.16% -> 14.20% (+0.04%) флэт
  - inside_volatile: 12.01% -> 11.54% (-0.46%) флэт
  - fastutil: 8.54% -> 9.12% (+0.59%) флэт
  - java_util: 7.01% -> 6.75% (-0.27%) флэт
  - paletted: 6.41% -> 6.19% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
