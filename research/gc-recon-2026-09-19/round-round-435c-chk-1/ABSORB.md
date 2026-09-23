# absorb ROUND (round-435c-chk-1, run 35924777490, branch round-435c-chk-1, head a17cde0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7056336 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 7056336 (поллов=6); TPS_exp=2.28; normalized=+7.2%
- GC: young=104, Full=9, total=19.4s, avg=172ms, max=2578ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103377 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.08% (-0.64%) флэт
  - broadphase: 15.66% -> 9.60% (-6.06%) спад
  - nav_ai: 14.16% -> 3.13% (-11.03%) спад
  - inside_volatile: 12.01% -> 15.55% (+3.55%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.21%) спад
  - java_util: 7.01% -> 8.48% (+1.47%) РОСТ
  - paletted: 6.41% -> 5.30% (-1.10%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
