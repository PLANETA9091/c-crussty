# absorb ROUND (a8, run 36104771871, branch round-455-anchor-8, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7114983 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 7114983 (поллов=6); TPS_exp=2.30; normalized=-2.1%
- GC: young=106, Full=9, total=20.7s, avg=180ms, max=2683ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116263 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.47% (-1.70%) спад
  - fluid: 16.72% -> 15.75% (-0.97%) флэт
  - broadphase: 15.66% -> 14.74% (-0.91%) флэт
  - nav_ai: 14.16% -> 14.27% (+0.11%) флэт
  - inside_volatile: 12.01% -> 11.11% (-0.90%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.68% (-0.34%) флэт
  - paletted: 6.41% -> 6.00% (-0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
