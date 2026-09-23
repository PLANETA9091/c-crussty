# absorb ROUND (round-432-anchor-i, run 35896516315, branch round-432-anchor-i, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6787643 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6787643 (поллов=6); TPS_exp=2.23; normalized=+1.0%
- GC: young=112, Full=10, total=26.5s, avg=218ms, max=2630ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115611 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.50% (-1.67%) спад
  - fluid: 16.72% -> 16.73% (+0.01%) флэт
  - broadphase: 15.66% -> 14.42% (-1.24%) спад
  - nav_ai: 14.16% -> 12.88% (-1.28%) спад
  - inside_volatile: 12.01% -> 10.86% (-1.15%) спад
  - fastutil: 8.54% -> 8.42% (-0.11%) флэт
  - java_util: 7.01% -> 6.63% (-0.38%) флэт
  - paletted: 6.41% -> 7.04% (+0.63%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
