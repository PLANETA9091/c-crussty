# absorb ROUND (anchor-n2, run 35914687324, branch round-434-anchor-n2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6851031 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.80 @ 6851031 (поллов=6); TPS_exp=2.24; normalized=-19.7%
- GC: young=104, Full=9, total=21.5s, avg=190ms, max=2512ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115965 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.80% (-2.37%) спад
  - fluid: 16.72% -> 15.40% (-1.32%) спад
  - broadphase: 15.66% -> 15.46% (-0.19%) флэт
  - nav_ai: 14.16% -> 13.54% (-0.62%) флэт
  - inside_volatile: 12.01% -> 10.98% (-1.03%) спад
  - fastutil: 8.54% -> 8.37% (-0.16%) флэт
  - java_util: 7.01% -> 6.44% (-0.58%) флэт
  - paletted: 6.41% -> 5.93% (-0.48%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
