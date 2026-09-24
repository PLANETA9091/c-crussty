# absorb ROUND (round-443g-anchor-7, run 36042406730, branch round-443g-anchor-7, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6548665 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6548665 (поллов=6); TPS_exp=2.18; normalized=-3.6%
- GC: young=109, Full=10, total=23.9s, avg=201ms, max=2490ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116144 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.88% (-1.29%) спад
  - fluid: 16.72% -> 15.99% (-0.72%) флэт
  - broadphase: 15.66% -> 15.62% (-0.03%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.67%) флэт
  - inside_volatile: 12.01% -> 11.16% (-0.85%) флэт
  - fastutil: 8.54% -> 9.04% (+0.51%) флэт
  - java_util: 7.01% -> 6.80% (-0.22%) флэт
  - paletted: 6.41% -> 6.01% (-0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
