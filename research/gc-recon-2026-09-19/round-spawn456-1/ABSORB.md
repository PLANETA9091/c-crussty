# absorb ROUND (spawn456-1, run 36113995075, branch round-456a-spawn-1, head c228e05)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7247637 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 7247637 (поллов=6); TPS_exp=2.33; normalized=+14.0%
- GC: young=105, Full=9, total=20.4s, avg=179ms, max=2971ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105721 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.84% (-0.87%) флэт
  - broadphase: 15.66% -> 9.55% (-6.11%) спад
  - nav_ai: 14.16% -> 3.16% (-11.00%) спад
  - inside_volatile: 12.01% -> 16.75% (+4.74%) РОСТ
  - fastutil: 8.54% -> 6.78% (-1.75%) спад
  - java_util: 7.01% -> 8.63% (+1.62%) РОСТ
  - paletted: 6.41% -> 5.06% (-1.34%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
