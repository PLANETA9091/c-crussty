# absorb ROUND (451c-chunk-8, run 36070508326, branch round-451c-chunk-8, head 29ac648)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7018843 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7018843 (поллов=5); TPS_exp=2.28; normalized=+9.8%
- GC: young=110, Full=9, total=19.9s, avg=167ms, max=2499ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105553 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.79% (+0.07%) флэт
  - broadphase: 15.66% -> 9.80% (-5.86%) спад
  - nav_ai: 14.16% -> 2.94% (-11.22%) спад
  - inside_volatile: 12.01% -> 16.63% (+4.63%) РОСТ
  - fastutil: 8.54% -> 6.68% (-1.86%) спад
  - java_util: 7.01% -> 8.87% (+1.86%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.06%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
