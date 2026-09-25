# absorb ROUND (a30-457, run 36135863809, branch round-457-anchor-30, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6669205 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 6669205 (поллов=5); TPS_exp=2.20; normalized=-13.8%
- GC: young=107, Full=9, total=20.8s, avg=179ms, max=2420ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116311 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.98% (-2.20%) спад
  - fluid: 16.72% -> 15.55% (-1.16%) спад
  - broadphase: 15.66% -> 14.66% (-1.00%) спад
  - nav_ai: 14.16% -> 13.46% (-0.70%) флэт
  - inside_volatile: 12.01% -> 10.90% (-1.11%) спад
  - fastutil: 8.54% -> 8.35% (-0.19%) флэт
  - java_util: 7.01% -> 6.16% (-0.85%) флэт
  - paletted: 6.41% -> 6.13% (-0.28%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
