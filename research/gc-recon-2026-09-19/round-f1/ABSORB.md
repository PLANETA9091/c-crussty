# absorb ROUND (f1, run 35753013243, branch round-416-c-f1, head 8e47eb0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8816403 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.30 @ 8816403 (поллов=5); TPS_exp=2.66; normalized=+24.3%
- GC: young=1286, Full=10, total=26.9s, avg=21ms, max=2024ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108187 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 19.40% (+2.69%) РОСТ
  - broadphase: 15.66% -> 13.26% (-2.40%) спад
  - nav_ai: 14.16% -> 9.35% (-4.81%) спад
  - inside_volatile: 12.01% -> 11.13% (-0.87%) флэт
  - fastutil: 8.54% -> 8.16% (-0.38%) флэт
  - java_util: 7.01% -> 7.83% (+0.82%) флэт
  - paletted: 6.41% -> 5.77% (-0.63%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
