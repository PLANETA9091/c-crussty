# absorb ROUND (451a-anchor-6, run 36069478227, branch round-451-anchor-6, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6979582 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6979582 (поллов=5); TPS_exp=2.27; normalized=-7.4%
- GC: young=113, Full=10, total=23.4s, avg=191ms, max=2301ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116244 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.44% (-1.74%) спад
  - fluid: 16.72% -> 15.59% (-1.13%) спад
  - broadphase: 15.66% -> 14.39% (-1.27%) спад
  - nav_ai: 14.16% -> 13.74% (-0.43%) флэт
  - inside_volatile: 12.01% -> 11.76% (-0.25%) флэт
  - fastutil: 8.54% -> 8.58% (+0.04%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.35% (-0.05%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
