# absorb ROUND (round-437b-pd-inf1, run 35938808401, branch round-437b-pd-inf1, head d8c8463)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6722009 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=2.30 @ 6722009 (поллов=5); TPS_exp=2.21; normalized=+3.9%
- GC: young=113, Full=10, total=26.5s, avg=215ms, max=2524ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115787 сэмплов (базлайн 115655)
  - items: 31.17% -> 27.98% (-3.19%) спад
  - fluid: 16.72% -> 15.89% (-0.83%) флэт
  - broadphase: 15.66% -> 14.63% (-1.02%) спад
  - nav_ai: 14.16% -> 13.49% (-0.67%) флэт
  - inside_volatile: 12.01% -> 10.80% (-1.20%) спад
  - fastutil: 8.54% -> 8.01% (-0.53%) флэт
  - java_util: 7.01% -> 6.20% (-0.81%) флэт
  - paletted: 6.41% -> 6.84% (+0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **CRASH-REFUTED**
