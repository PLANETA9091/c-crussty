# absorb ROUND (c424l3, run 35827088941, branch round-424-c-l3, head 6f92ea7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8711952 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 8711952 (поллов=5); TPS_exp=2.63; normalized=+17.7%
- GC: young=1239, Full=10, total=27.0s, avg=22ms, max=2172ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105477 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.44% (-0.28%) флэт
  - broadphase: 15.66% -> 14.00% (-1.65%) спад
  - nav_ai: 14.16% -> 6.47% (-7.69%) спад
  - inside_volatile: 12.01% -> 11.08% (-0.92%) флэт
  - fastutil: 8.54% -> 7.09% (-1.45%) спад
  - java_util: 7.01% -> 7.90% (+0.89%) флэт
  - paletted: 6.41% -> 6.02% (-0.39%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
