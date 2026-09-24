# absorb ROUND (round-437-ins4-2r2, run 35938730547, branch round-437-ins4-2r2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8583329 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8583329 (поллов=5); TPS_exp=2.61; normalized=+22.8%
- GC: young=128, Full=9, total=18.1s, avg=132ms, max=2043ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103800 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.45% (-0.27%) флэт
  - broadphase: 15.66% -> 9.00% (-6.66%) спад
  - nav_ai: 14.16% -> 3.97% (-10.19%) спад
  - inside_volatile: 12.01% -> 15.53% (+3.53%) РОСТ
  - fastutil: 8.54% -> 6.30% (-2.24%) спад
  - java_util: 7.01% -> 8.66% (+1.65%) РОСТ
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
