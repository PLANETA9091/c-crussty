# absorb ROUND (anchor-3, run 36093414018, branch round-454-anchor-3, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6677719 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6677719 (поллов=5); TPS_exp=2.21; normalized=-4.8%
- GC: young=106, Full=9, total=20.2s, avg=176ms, max=2361ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116851 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.21% (-1.96%) спад
  - fluid: 16.72% -> 15.74% (-0.97%) флэт
  - broadphase: 15.66% -> 16.08% (+0.43%) флэт
  - nav_ai: 14.16% -> 14.17% (+0.00%) флэт
  - inside_volatile: 12.01% -> 11.14% (-0.86%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.73% (-0.29%) флэт
  - paletted: 6.41% -> 6.23% (-0.17%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
