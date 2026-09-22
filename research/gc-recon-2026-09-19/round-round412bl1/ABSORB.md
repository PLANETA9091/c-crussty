# absorb ROUND (round412bl1, run 35700016663, branch round-412-b-l1, head e555178)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6595922 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=0.70 @ 6595922 (поллов=5); TPS_exp=2.19; normalized=-68.0%
- GC: young=74, Full=9, total=15.7s, avg=190ms, max=2324ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=42855 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.23% (-2.95%) спад
  - fluid: 16.72% -> 16.45% (-0.26%) флэт
  - broadphase: 15.66% -> 14.44% (-1.21%) спад
  - nav_ai: 14.16% -> 10.96% (-3.20%) спад
  - inside_volatile: 12.01% -> 9.73% (-2.28%) спад
  - fastutil: 8.54% -> 7.76% (-0.78%) флэт
  - java_util: 7.01% -> 6.86% (-0.15%) флэт
  - paletted: 6.41% -> 6.80% (+0.39%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
