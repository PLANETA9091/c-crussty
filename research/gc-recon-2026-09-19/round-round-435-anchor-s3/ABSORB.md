# absorb ROUND (round-435-anchor-s3, run 35924444946, branch round-435-anchor-s3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6761558 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6761558 (поллов=6); TPS_exp=2.22; normalized=+10.2%
- GC: young=112, Full=9, total=24.0s, avg=198ms, max=2768ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116334 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.53% (-1.64%) спад
  - fluid: 16.72% -> 16.47% (-0.24%) флэт
  - broadphase: 15.66% -> 14.89% (-0.76%) флэт
  - nav_ai: 14.16% -> 13.62% (-0.54%) флэт
  - inside_volatile: 12.01% -> 11.06% (-0.94%) флэт
  - fastutil: 8.54% -> 8.77% (+0.23%) флэт
  - java_util: 7.01% -> 6.42% (-0.59%) флэт
  - paletted: 6.41% -> 6.87% (+0.47%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
