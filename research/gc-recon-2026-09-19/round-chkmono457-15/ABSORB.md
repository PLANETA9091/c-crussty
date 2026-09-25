# absorb ROUND (chkmono457-15, run 36146771631, branch round-456c-chunkmono-15, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6215978 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6215978 (поллов=6); TPS_exp=2.11; normalized=+13.9%
- GC: young=103, Full=9, total=18.5s, avg=165ms, max=2385ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105255 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.23% (-0.48%) флэт
  - broadphase: 15.66% -> 9.61% (-6.04%) спад
  - nav_ai: 14.16% -> 3.07% (-11.09%) спад
  - inside_volatile: 12.01% -> 16.28% (+4.27%) РОСТ
  - fastutil: 8.54% -> 6.40% (-2.13%) спад
  - java_util: 7.01% -> 8.28% (+1.27%) РОСТ
  - paletted: 6.41% -> 5.34% (-1.06%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
