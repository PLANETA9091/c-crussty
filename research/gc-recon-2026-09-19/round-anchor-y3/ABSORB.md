# absorb ROUND (anchor-y3, run 35930420872, branch round-436-anchor-y3, head e05994c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7020249 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 7020249 (поллов=6); TPS_exp=2.28; normalized=-1.2%
- GC: young=106, Full=9, total=21.1s, avg=184ms, max=2821ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115839 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.74% (-1.43%) спад
  - fluid: 16.72% -> 16.07% (-0.64%) флэт
  - broadphase: 15.66% -> 15.83% (+0.17%) флэт
  - nav_ai: 14.16% -> 14.02% (-0.14%) флэт
  - inside_volatile: 12.01% -> 11.33% (-0.67%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 6.59% (-0.42%) флэт
  - paletted: 6.41% -> 6.36% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
