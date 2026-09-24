# absorb ROUND (450-anchor-12, run 36050652433, branch round-450-anchor-12, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7177820 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7177820 (поллов=5); TPS_exp=2.31; normalized=-4.8%
- GC: young=108, Full=10, total=23.7s, avg=201ms, max=2592ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116542 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.03% (-1.14%) спад
  - fluid: 16.72% -> 15.75% (-0.97%) флэт
  - broadphase: 15.66% -> 15.26% (-0.40%) флэт
  - nav_ai: 14.16% -> 13.92% (-0.24%) флэт
  - inside_volatile: 12.01% -> 11.57% (-0.44%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 6.92% (-0.10%) флэт
  - paletted: 6.41% -> 6.23% (-0.17%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
