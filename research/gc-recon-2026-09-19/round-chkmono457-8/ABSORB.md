# absorb ROUND (chkmono457-8, run 36131776794, branch round-456c-chunkmono-8, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7084301 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7084301 (поллов=5); TPS_exp=2.29; normalized=+9.1%
- GC: young=101, Full=9, total=19.0s, avg=173ms, max=2494ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105480 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.06% (-0.65%) флэт
  - broadphase: 15.66% -> 9.81% (-5.84%) спад
  - nav_ai: 14.16% -> 2.96% (-11.20%) спад
  - inside_volatile: 12.01% -> 16.30% (+4.29%) РОСТ
  - fastutil: 8.54% -> 6.63% (-1.91%) спад
  - java_util: 7.01% -> 8.89% (+1.88%) РОСТ
  - paletted: 6.41% -> 5.24% (-1.17%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
