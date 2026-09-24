# absorb ROUND (round-443g-anchor-4, run 36038537162, branch round-443g-anchor-4, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6310617 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6310617 (поллов=6); TPS_exp=2.13; normalized=+8.1%
- GC: young=112, Full=9, total=21.0s, avg=173ms, max=2681ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116456 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.39% (-1.78%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 15.15% (-0.51%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 11.42% (-0.59%) флэт
  - fastutil: 8.54% -> 8.76% (+0.23%) флэт
  - java_util: 7.01% -> 6.58% (-0.44%) флэт
  - paletted: 6.41% -> 6.12% (-0.28%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
