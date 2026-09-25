# absorb ROUND (chkmono456-5, run 36125892041, branch round-456c-chunkmono-5, head cf620df)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7209045 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7209045 (поллов=5); TPS_exp=2.32; normalized=+7.9%
- GC: young=105, Full=9, total=18.9s, avg=166ms, max=2432ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105194 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.98% (-0.73%) флэт
  - broadphase: 15.66% -> 10.27% (-5.39%) спад
  - nav_ai: 14.16% -> 3.12% (-11.04%) спад
  - inside_volatile: 12.01% -> 16.18% (+4.17%) РОСТ
  - fastutil: 8.54% -> 6.52% (-2.02%) спад
  - java_util: 7.01% -> 8.25% (+1.24%) РОСТ
  - paletted: 6.41% -> 5.39% (-1.02%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
