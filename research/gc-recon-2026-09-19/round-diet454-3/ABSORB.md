# absorb ROUND (diet454-3, run 36094617772, branch round-454c-diet-3, head 451c4e4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6667438 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 6667438 (поллов=6); TPS_exp=2.20; normalized=-2.4%
- GC: young=108, Full=9, total=22.1s, avg=189ms, max=2947ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116628 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.40% (-1.77%) спад
  - fluid: 16.72% -> 15.86% (-0.85%) флэт
  - broadphase: 15.66% -> 15.53% (-0.12%) флэт
  - nav_ai: 14.16% -> 13.94% (-0.22%) флэт
  - inside_volatile: 12.01% -> 11.10% (-0.90%) флэт
  - fastutil: 8.54% -> 8.61% (+0.07%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 5.99% (-0.42%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
