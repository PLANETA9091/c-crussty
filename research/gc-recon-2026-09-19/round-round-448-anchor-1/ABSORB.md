# absorb ROUND (round-448-anchor-1, run 36015009093, branch round-448-anchor-1, head 97afb42)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8626874 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 8626874 (поллов=5); TPS_exp=2.62; normalized=-15.9%
- GC: young=113, Full=9, total=24.9s, avg=204ms, max=2895ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116738 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.49% (-1.68%) спад
  - fluid: 16.72% -> 17.64% (+0.92%) флэт
  - broadphase: 15.66% -> 15.10% (-0.56%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.49%) флэт
  - inside_volatile: 12.01% -> 11.30% (-0.70%) флэт
  - fastutil: 8.54% -> 8.64% (+0.10%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 7.50% (+1.09%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
