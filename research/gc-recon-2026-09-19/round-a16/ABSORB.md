# absorb ROUND (a16, run 36104843453, branch round-455-anchor-16, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7214631 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7214631 (поллов=6); TPS_exp=2.32; normalized=-5.1%
- GC: young=113, Full=9, total=21.2s, avg=174ms, max=2456ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116503 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.98% (-2.19%) спад
  - fluid: 16.72% -> 15.95% (-0.77%) флэт
  - broadphase: 15.66% -> 15.37% (-0.29%) флэт
  - nav_ai: 14.16% -> 14.10% (-0.06%) флэт
  - inside_volatile: 12.01% -> 11.51% (-0.50%) флэт
  - fastutil: 8.54% -> 8.65% (+0.12%) флэт
  - java_util: 7.01% -> 6.79% (-0.22%) флэт
  - paletted: 6.41% -> 6.05% (-0.36%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
