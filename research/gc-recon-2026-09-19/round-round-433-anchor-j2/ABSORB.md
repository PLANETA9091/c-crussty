# absorb ROUND (round-433-anchor-j2, run 35910370054, branch round-433-anchor-j2, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6807082 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6807082 (поллов=6); TPS_exp=2.23; normalized=+5.3%
- GC: young=113, Full=10, total=23.7s, avg=193ms, max=2396ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117471 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.71% (-1.46%) спад
  - fluid: 16.72% -> 16.11% (-0.61%) флэт
  - broadphase: 15.66% -> 14.77% (-0.89%) флэт
  - nav_ai: 14.16% -> 13.61% (-0.55%) флэт
  - inside_volatile: 12.01% -> 11.71% (-0.29%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.55% (-0.46%) флэт
  - paletted: 6.41% -> 6.12% (-0.28%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
