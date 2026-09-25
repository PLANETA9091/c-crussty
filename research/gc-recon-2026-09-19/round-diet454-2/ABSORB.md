# absorb ROUND (diet454-2, run 36094610372, branch round-454c-diet-2, head 451c4e4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6965938 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6965938 (поллов=5); TPS_exp=2.27; normalized=-2.9%
- GC: young=114, Full=10, total=24.2s, avg=195ms, max=2413ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116347 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.91% (-1.26%) спад
  - fluid: 16.72% -> 15.96% (-0.76%) флэт
  - broadphase: 15.66% -> 15.24% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.81% (-0.35%) флэт
  - inside_volatile: 12.01% -> 11.73% (-0.28%) флэт
  - fastutil: 8.54% -> 8.87% (+0.33%) флэт
  - java_util: 7.01% -> 6.71% (-0.30%) флэт
  - paletted: 6.41% -> 6.32% (-0.09%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
