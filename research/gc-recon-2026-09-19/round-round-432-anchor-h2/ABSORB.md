# absorb ROUND (round-432-anchor-h2, run 35896475668, branch round-432-anchor-h2, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8911660 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8911660 (поллов=5); TPS_exp=2.68; normalized=+0.9%
- GC: young=123, Full=10, total=22.4s, avg=169ms, max=2167ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112837 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.56% (-2.61%) спад
  - fluid: 16.72% -> 16.60% (-0.12%) флэт
  - broadphase: 15.66% -> 15.68% (+0.02%) флэт
  - nav_ai: 14.16% -> 13.98% (-0.18%) флэт
  - inside_volatile: 12.01% -> 10.61% (-1.39%) спад
  - fastutil: 8.54% -> 8.77% (+0.23%) флэт
  - java_util: 7.01% -> 6.28% (-0.73%) флэт
  - paletted: 6.41% -> 7.05% (+0.64%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
