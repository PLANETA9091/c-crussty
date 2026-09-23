# absorb ROUND (round-432-anchor-e, run 35891357067, branch round-432-anchor-e, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6853578 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6853578 (поллов=5); TPS_exp=2.24; normalized=-1.9%
- GC: young=115, Full=9, total=21.0s, avg=170ms, max=2476ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116654 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.85% (-1.32%) спад
  - fluid: 16.72% -> 16.03% (-0.68%) флэт
  - broadphase: 15.66% -> 14.79% (-0.86%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.45%) флэт
  - inside_volatile: 12.01% -> 11.89% (-0.12%) флэт
  - fastutil: 8.54% -> 8.61% (+0.08%) флэт
  - java_util: 7.01% -> 8.56% (+1.55%) РОСТ
  - paletted: 6.41% -> 6.18% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
