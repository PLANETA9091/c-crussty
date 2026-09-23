# absorb ROUND (430-anchor-f, run 35873227247, branch round-430-anchor-f, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8463234 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8463234 (поллов=5); TPS_exp=2.58; normalized=-7.0%
- GC: young=123, Full=10, total=22.1s, avg=167ms, max=2286ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113013 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.23% (-1.94%) спад
  - fluid: 16.72% -> 16.91% (+0.19%) флэт
  - broadphase: 15.66% -> 14.86% (-0.79%) флэт
  - nav_ai: 14.16% -> 13.63% (-0.54%) флэт
  - inside_volatile: 12.01% -> 10.70% (-1.31%) спад
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.41% (-0.61%) флэт
  - paletted: 6.41% -> 6.88% (+0.48%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
