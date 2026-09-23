# absorb ROUND (chunk421l2, run 35804832150, branch round-421-c-chunk-l2, head 600e3cc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8527940 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8527940 (поллов=5); TPS_exp=2.59; normalized=+7.9%
- GC: young=123, Full=10, total=20.9s, avg=157ms, max=2133ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113361 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.43% (-2.74%) спад
  - fluid: 16.72% -> 16.17% (-0.54%) флэт
  - broadphase: 15.66% -> 14.99% (-0.66%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 10.90% (-1.11%) спад
  - fastutil: 8.54% -> 9.02% (+0.48%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 6.71% (+0.30%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
