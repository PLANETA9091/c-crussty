# absorb ROUND (round-441-pd-2r, run 35954875004, branch round-441-pd-2r, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6862931 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6862931 (поллов=5); TPS_exp=2.24; normalized=-10.9%
- GC: young=112, Full=10, total=24.6s, avg=202ms, max=2424ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117120 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.45% (-1.72%) спад
  - fluid: 16.72% -> 16.05% (-0.67%) флэт
  - broadphase: 15.66% -> 14.91% (-0.75%) флэт
  - nav_ai: 14.16% -> 12.96% (-1.20%) спад
  - inside_volatile: 12.01% -> 11.50% (-0.51%) флэт
  - fastutil: 8.54% -> 8.19% (-0.35%) флэт
  - java_util: 7.01% -> 6.24% (-0.77%) флэт
  - paletted: 6.41% -> 6.75% (+0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
