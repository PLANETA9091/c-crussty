# absorb ROUND (bqb, run 35764275770, branch round-417-c-bqb, head bad02d8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7242099 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.30 @ 7242099 (поллов=5); TPS_exp=2.32; normalized=+42.0%
- GC: young=1143, Full=9, total=28.3s, avg=25ms, max=2257ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108278 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.84% (+1.12%) РОСТ
  - broadphase: 15.66% -> 14.50% (-1.16%) спад
  - nav_ai: 14.16% -> 9.26% (-4.90%) спад
  - inside_volatile: 12.01% -> 12.56% (+0.55%) флэт
  - fastutil: 8.54% -> 7.23% (-1.30%) спад
  - java_util: 7.01% -> 7.70% (+0.69%) флэт
  - paletted: 6.41% -> 5.86% (-0.55%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
