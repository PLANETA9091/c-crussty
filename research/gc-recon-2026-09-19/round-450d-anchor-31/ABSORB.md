# absorb ROUND (450d-anchor-31, run 36063337933, branch round-450-anchor-31, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7380343 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 7380343 (поллов=6); TPS_exp=2.35; normalized=+19.0%
- GC: young=115, Full=9, total=23.7s, avg=191ms, max=3186ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116810 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.23% (-0.94%) флэт
  - fluid: 16.72% -> 18.27% (+1.55%) РОСТ
  - broadphase: 15.66% -> 15.90% (+0.25%) флэт
  - nav_ai: 14.16% -> 14.05% (-0.11%) флэт
  - inside_volatile: 12.01% -> 12.23% (+0.22%) флэт
  - fastutil: 8.54% -> 8.83% (+0.29%) флэт
  - java_util: 7.01% -> 5.96% (-1.05%) спад
  - paletted: 6.41% -> 6.49% (+0.09%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
