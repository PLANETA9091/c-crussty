# absorb ROUND (round-449-anchor-3, run 36043756339, branch round-449-anchor-3, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7078713 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7078713 (поллов=6); TPS_exp=2.29; normalized=+4.8%
- GC: young=118, Full=9, total=21.4s, avg=168ms, max=2360ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116792 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.44% (-1.73%) спад
  - fluid: 16.72% -> 15.75% (-0.97%) флэт
  - broadphase: 15.66% -> 14.97% (-0.69%) флэт
  - nav_ai: 14.16% -> 14.09% (-0.07%) флэт
  - inside_volatile: 12.01% -> 11.73% (-0.28%) флэт
  - fastutil: 8.54% -> 8.58% (+0.04%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 5.96% (-0.44%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
