# absorb ROUND (a12-457, run 36131700115, branch round-457-anchor-12, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6606571 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6606571 (поллов=5); TPS_exp=2.19; normalized=+5.0%
- GC: young=109, Full=9, total=21.6s, avg=183ms, max=2545ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115139 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.59% (-1.58%) спад
  - fluid: 16.72% -> 16.69% (-0.02%) флэт
  - broadphase: 15.66% -> 14.89% (-0.77%) флэт
  - nav_ai: 14.16% -> 13.77% (-0.39%) флэт
  - inside_volatile: 12.01% -> 11.09% (-0.91%) флэт
  - fastutil: 8.54% -> 9.08% (+0.54%) флэт
  - java_util: 7.01% -> 6.81% (-0.21%) флэт
  - paletted: 6.41% -> 7.33% (+0.93%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
