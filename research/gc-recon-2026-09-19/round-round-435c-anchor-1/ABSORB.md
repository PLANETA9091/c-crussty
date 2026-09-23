# absorb ROUND (round-435c-anchor-1, run 35924765731, branch round-435c-anchor-1, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8538611 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8538611 (поллов=5); TPS_exp=2.60; normalized=+0.1%
- GC: young=122, Full=10, total=21.8s, avg=165ms, max=2169ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113719 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.90% (-2.27%) спад
  - fluid: 16.72% -> 16.92% (+0.20%) флэт
  - broadphase: 15.66% -> 14.90% (-0.75%) флэт
  - nav_ai: 14.16% -> 13.87% (-0.29%) флэт
  - inside_volatile: 12.01% -> 10.87% (-1.14%) спад
  - fastutil: 8.54% -> 9.04% (+0.51%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 7.06% (+0.65%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
