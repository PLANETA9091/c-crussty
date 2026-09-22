# absorb ROUND (anchor420b, run 35788816648, branch round-420-anchorb, head 41456af)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6827107 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6827107 (поллов=5); TPS_exp=2.24; normalized=-1.6%
- GC: young=109, Full=10, total=23.3s, avg=196ms, max=2406ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116136 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.70% (-1.47%) спад
  - fluid: 16.72% -> 15.92% (-0.80%) флэт
  - broadphase: 15.66% -> 14.84% (-0.82%) флэт
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 11.34% (-0.66%) флэт
  - fastutil: 8.54% -> 8.48% (-0.06%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.29% (-0.11%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
