# absorb ROUND (round-432-anchor-f2, run 35896462176, branch round-432-anchor-f2, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6969636 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6969636 (поллов=5); TPS_exp=2.27; normalized=-7.4%
- GC: young=106, Full=10, total=23.1s, avg=199ms, max=2436ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115933 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.88% (-1.29%) спад
  - fluid: 16.72% -> 15.70% (-1.01%) спад
  - broadphase: 15.66% -> 15.09% (-0.56%) флэт
  - nav_ai: 14.16% -> 13.54% (-0.62%) флэт
  - inside_volatile: 12.01% -> 11.26% (-0.75%) флэт
  - fastutil: 8.54% -> 8.63% (+0.10%) флэт
  - java_util: 7.01% -> 6.58% (-0.44%) флэт
  - paletted: 6.41% -> 6.10% (-0.31%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
