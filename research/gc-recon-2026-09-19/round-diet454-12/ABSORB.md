# absorb ROUND (diet454-12, run 36100450710, branch round-454c-diet-12, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6744409 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6744409 (поллов=6); TPS_exp=2.22; normalized=+21.7%
- GC: young=101, Full=9, total=18.9s, avg=171ms, max=2509ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104517 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.42% (-0.29%) флэт
  - broadphase: 15.66% -> 10.12% (-5.54%) спад
  - nav_ai: 14.16% -> 3.18% (-10.98%) спад
  - inside_volatile: 12.01% -> 16.60% (+4.60%) РОСТ
  - fastutil: 8.54% -> 6.28% (-2.26%) спад
  - java_util: 7.01% -> 8.49% (+1.47%) РОСТ
  - paletted: 6.41% -> 5.12% (-1.28%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
