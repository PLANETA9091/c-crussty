# absorb ROUND (450e-anchor-46, run 36066530226, branch round-450-anchor-46, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6649702 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6649702 (поллов=5); TPS_exp=2.20; normalized=-4.5%
- GC: young=110, Full=8, total=21.3s, avg=181ms, max=2452ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117003 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.62% (-1.55%) спад
  - fluid: 16.72% -> 16.19% (-0.53%) флэт
  - broadphase: 15.66% -> 15.49% (-0.16%) флэт
  - nav_ai: 14.16% -> 14.04% (-0.13%) флэт
  - inside_volatile: 12.01% -> 11.70% (-0.31%) флэт
  - fastutil: 8.54% -> 8.70% (+0.17%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.14% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
