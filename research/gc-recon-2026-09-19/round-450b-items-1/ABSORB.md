# absorb ROUND (450b-items-1, run 36054213025, branch round-450b-items-1, head f33b865)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6979160 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6979160 (поллов=5); TPS_exp=2.27; normalized=+5.8%
- GC: young=111, Full=10, total=21.7s, avg=179ms, max=2344ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115827 сэмплов (базлайн 115655)
  - items: 31.17% -> 25.41% (-5.76%) спад
  - fluid: 16.72% -> 14.97% (-1.75%) спад
  - broadphase: 15.66% -> 13.62% (-2.04%) спад
  - nav_ai: 14.16% -> 14.50% (+0.33%) флэт
  - inside_volatile: 12.01% -> 11.92% (-0.09%) флэт
  - fastutil: 8.54% -> 8.78% (+0.24%) флэт
  - java_util: 7.01% -> 6.79% (-0.23%) флэт
  - paletted: 6.41% -> 5.82% (-0.58%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
