# absorb ROUND (a2-456, run 36112088121, branch round-456-anchor-2, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7003062 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7003062 (поллов=5); TPS_exp=2.27; normalized=-3.2%
- GC: young=113, Full=9, total=21.3s, avg=175ms, max=2611ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117490 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.20% (-1.97%) спад
  - fluid: 16.72% -> 15.61% (-1.11%) спад
  - broadphase: 15.66% -> 16.17% (+0.52%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.37%) флэт
  - inside_volatile: 12.01% -> 11.81% (-0.20%) флэт
  - fastutil: 8.54% -> 8.84% (+0.31%) флэт
  - java_util: 7.01% -> 6.84% (-0.17%) флэт
  - paletted: 6.41% -> 6.05% (-0.36%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
