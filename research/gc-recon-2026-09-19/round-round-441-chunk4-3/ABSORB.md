# absorb ROUND (round-441-chunk4-3, run 35954854450, branch round-441-chunk4-3, head c5fe025)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6342096 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6342096 (поллов=5); TPS_exp=2.13; normalized=+17.1%
- GC: young=106, Full=9, total=21.4s, avg=186ms, max=2729ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104380 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.89% (+0.17%) флэт
  - broadphase: 15.66% -> 9.32% (-6.34%) спад
  - nav_ai: 14.16% -> 3.18% (-10.99%) спад
  - inside_volatile: 12.01% -> 15.71% (+3.71%) РОСТ
  - fastutil: 8.54% -> 6.61% (-1.93%) спад
  - java_util: 7.01% -> 9.12% (+2.11%) РОСТ
  - paletted: 6.41% -> 6.08% (-0.33%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
