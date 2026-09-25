# absorb ROUND (a9-457, run 36131665543, branch round-457-anchor-9, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6885976 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6885976 (поллов=6); TPS_exp=2.25; normalized=-6.6%
- GC: young=111, Full=9, total=20.9s, avg=175ms, max=2365ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117067 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.79% (-1.38%) спад
  - fluid: 16.72% -> 15.62% (-1.10%) спад
  - broadphase: 15.66% -> 15.14% (-0.52%) флэт
  - nav_ai: 14.16% -> 13.69% (-0.47%) флэт
  - inside_volatile: 12.01% -> 11.59% (-0.41%) флэт
  - fastutil: 8.54% -> 8.64% (+0.10%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 6.05% (-0.35%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
