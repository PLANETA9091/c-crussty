# absorb ROUND (chkmono457-12, run 36143863797, branch round-456c-chunkmono-12, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7053858 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7053858 (поллов=5); TPS_exp=2.28; normalized=-3.7%
- GC: young=101, Full=9, total=19.8s, avg=180ms, max=2696ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104260 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.41% (-0.31%) флэт
  - broadphase: 15.66% -> 9.73% (-5.92%) спад
  - nav_ai: 14.16% -> 3.30% (-10.86%) спад
  - inside_volatile: 12.01% -> 16.30% (+4.29%) РОСТ
  - fastutil: 8.54% -> 6.44% (-2.10%) спад
  - java_util: 7.01% -> 8.73% (+1.71%) РОСТ
  - paletted: 6.41% -> 5.18% (-1.22%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
