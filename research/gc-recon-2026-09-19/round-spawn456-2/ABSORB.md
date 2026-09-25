# absorb ROUND (spawn456-2, run 36114000472, branch round-456a-spawn-2, head c228e05)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6806602 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6806602 (поллов=5); TPS_exp=2.23; normalized=+7.5%
- GC: young=104, Full=9, total=21.5s, avg=190ms, max=2639ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104541 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.29% (+0.57%) флэт
  - broadphase: 15.66% -> 9.32% (-6.34%) спад
  - nav_ai: 14.16% -> 3.21% (-10.95%) спад
  - inside_volatile: 12.01% -> 16.48% (+4.48%) РОСТ
  - fastutil: 8.54% -> 6.69% (-1.85%) спад
  - java_util: 7.01% -> 8.25% (+1.23%) РОСТ
  - paletted: 6.41% -> 6.04% (-0.37%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
