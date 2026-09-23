# absorb ROUND (round-432-anchor-c2, run 35896447016, branch round-432-anchor-c2, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6807627 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6807627 (поллов=5); TPS_exp=2.23; normalized=-1.5%
- GC: young=112, Full=10, total=24.2s, avg=198ms, max=2389ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116620 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.40% (-1.77%) спад
  - fluid: 16.72% -> 15.86% (-0.86%) флэт
  - broadphase: 15.66% -> 14.58% (-1.07%) спад
  - nav_ai: 14.16% -> 14.08% (-0.09%) флэт
  - inside_volatile: 12.01% -> 11.70% (-0.31%) флэт
  - fastutil: 8.54% -> 8.39% (-0.14%) флэт
  - java_util: 7.01% -> 6.58% (-0.43%) флэт
  - paletted: 6.41% -> 6.33% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
