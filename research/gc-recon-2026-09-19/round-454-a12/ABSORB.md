# absorb ROUND (454-a12, run 36093492149, branch round-454-anchor-12, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7163832 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 7163832 (поллов=5); TPS_exp=2.31; normalized=-13.3%
- GC: young=105, Full=9, total=19.9s, avg=175ms, max=2418ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116111 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.29% (-1.88%) спад
  - fluid: 16.72% -> 15.82% (-0.90%) флэт
  - broadphase: 15.66% -> 15.68% (+0.03%) флэт
  - nav_ai: 14.16% -> 14.03% (-0.13%) флэт
  - inside_volatile: 12.01% -> 11.13% (-0.88%) флэт
  - fastutil: 8.54% -> 8.98% (+0.45%) флэт
  - java_util: 7.01% -> 6.49% (-0.52%) флэт
  - paletted: 6.41% -> 6.25% (-0.16%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
