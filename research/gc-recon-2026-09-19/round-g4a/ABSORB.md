# absorb ROUND (g4a, run 35754098983, branch round-416-b-g4a, head 89b3bff)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7601085 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 7601085 (поллов=5); TPS_exp=2.40; normalized=+12.5%
- GC: young=113, Full=9, total=23.1s, avg=190ms, max=2757ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113238 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.89% (+0.17%) флэт
  - broadphase: 15.66% -> 15.83% (+0.17%) флэт
  - nav_ai: 14.16% -> 7.17% (-6.99%) спад
  - inside_volatile: 12.01% -> 12.48% (+0.47%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 5.93% (-0.48%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
