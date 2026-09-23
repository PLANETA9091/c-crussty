# absorb ROUND (round-432-anchor-b, run 35891274573, branch round-432-anchor-b, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8428308 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 8428308 (поллов=5); TPS_exp=2.57; normalized=-10.6%
- GC: young=114, Full=10, total=21.5s, avg=173ms, max=2204ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112714 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.46% (-2.71%) спад
  - fluid: 16.72% -> 16.27% (-0.44%) флэт
  - broadphase: 15.66% -> 15.38% (-0.28%) флэт
  - nav_ai: 14.16% -> 14.04% (-0.13%) флэт
  - inside_volatile: 12.01% -> 10.06% (-1.94%) спад
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.43% (-0.58%) флэт
  - paletted: 6.41% -> 6.74% (+0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
