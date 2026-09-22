# absorb ROUND (chunkc, run 35779704341, branch round-419-c-chc, head f7b0be4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7026302 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 7026302 (поллов=6); TPS_exp=2.28; normalized=+7.5%
- GC: young=112, Full=9, total=20.9s, avg=172ms, max=2331ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116143 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.26% (-1.91%) спад
  - fluid: 16.72% -> 15.49% (-1.22%) спад
  - broadphase: 15.66% -> 14.52% (-1.14%) спад
  - nav_ai: 14.16% -> 13.57% (-0.60%) флэт
  - inside_volatile: 12.01% -> 11.63% (-0.38%) флэт
  - fastutil: 8.54% -> 8.15% (-0.39%) флэт
  - java_util: 7.01% -> 6.95% (-0.06%) флэт
  - paletted: 6.41% -> 6.26% (-0.15%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
