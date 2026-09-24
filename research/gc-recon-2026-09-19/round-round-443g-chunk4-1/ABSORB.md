# absorb ROUND (round-443g-chunk4-1, run 36038497917, branch round-443g-chunk4-1, head c5fe025)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7607128 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7607128 (поллов=5); TPS_exp=2.40; normalized=+12.5%
- GC: young=116, Full=9, total=23.2s, avg=185ms, max=2914ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105524 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.94% (+2.22%) РОСТ
  - broadphase: 15.66% -> 10.13% (-5.53%) спад
  - nav_ai: 14.16% -> 3.48% (-10.68%) спад
  - inside_volatile: 12.01% -> 17.30% (+5.30%) РОСТ
  - fastutil: 8.54% -> 6.43% (-2.11%) спад
  - java_util: 7.01% -> 8.51% (+1.50%) РОСТ
  - paletted: 6.41% -> 5.58% (-0.83%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
