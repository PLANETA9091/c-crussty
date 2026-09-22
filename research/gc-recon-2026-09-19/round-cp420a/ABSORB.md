# absorb ROUND (cp420a, run 35791177411, branch round-420-a-cpa, head 9bde8bf)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6654194 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6654194 (поллов=5); TPS_exp=2.20; normalized=+18.2%
- GC: young=1151, Full=9, total=28.6s, avg=25ms, max=2435ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107379 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.11% (+0.39%) флэт
  - broadphase: 15.66% -> 14.28% (-1.37%) спад
  - nav_ai: 14.16% -> 9.47% (-4.69%) спад
  - inside_volatile: 12.01% -> 11.63% (-0.37%) флэт
  - fastutil: 8.54% -> 7.87% (-0.67%) флэт
  - java_util: 7.01% -> 7.92% (+0.91%) флэт
  - paletted: 6.41% -> 6.13% (-0.28%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
