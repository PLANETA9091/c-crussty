# absorb ROUND (451a-anchor-4, run 36069458885, branch round-451-anchor-4, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7086563 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7086563 (поллов=5); TPS_exp=2.29; normalized=-4.0%
- GC: young=111, Full=10, total=24.9s, avg=205ms, max=2689ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116786 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.99% (-1.18%) спад
  - fluid: 16.72% -> 15.80% (-0.91%) флэт
  - broadphase: 15.66% -> 15.87% (+0.22%) флэт
  - nav_ai: 14.16% -> 13.83% (-0.33%) флэт
  - inside_volatile: 12.01% -> 11.52% (-0.48%) флэт
  - fastutil: 8.54% -> 8.73% (+0.20%) флэт
  - java_util: 7.01% -> 6.68% (-0.33%) флэт
  - paletted: 6.41% -> 6.02% (-0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
