# absorb ROUND (round-437-anchor-2, run 35935285901, branch round-437-anchor-2, head e4d39b1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6624372 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6624372 (поллов=5); TPS_exp=2.19; normalized=+4.8%
- GC: young=112, Full=9, total=20.5s, avg=170ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117272 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.71% (-1.46%) спад
  - fluid: 16.72% -> 15.93% (-0.79%) флэт
  - broadphase: 15.66% -> 15.24% (-0.42%) флэт
  - nav_ai: 14.16% -> 13.84% (-0.32%) флэт
  - inside_volatile: 12.01% -> 11.69% (-0.31%) флэт
  - fastutil: 8.54% -> 8.68% (+0.14%) флэт
  - java_util: 7.01% -> 6.90% (-0.11%) флэт
  - paletted: 6.41% -> 6.32% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
