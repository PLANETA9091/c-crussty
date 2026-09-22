# absorb ROUND (chunka, run 35783030670, branch round-419-c-cha, head f7b0be4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7098787 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7098787 (поллов=5); TPS_exp=2.29; normalized=+0.3%
- GC: young=112, Full=10, total=24.7s, avg=202ms, max=2468ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116563 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.58% (-1.59%) спад
  - fluid: 16.72% -> 16.06% (-0.66%) флэт
  - broadphase: 15.66% -> 15.28% (-0.37%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 11.32% (-0.69%) флэт
  - fastutil: 8.54% -> 8.98% (+0.44%) флэт
  - java_util: 7.01% -> 6.90% (-0.11%) флэт
  - paletted: 6.41% -> 6.35% (-0.06%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
