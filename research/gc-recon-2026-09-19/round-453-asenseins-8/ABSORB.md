# absorb ROUND (453-asenseins-8, run 36090370416, branch round-453-asenseins-8, head 76d9798)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6636139 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6636139 (поллов=6); TPS_exp=2.20; normalized=+16.1%
- GC: young=105, Full=9, total=19.0s, avg=167ms, max=2478ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104477 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.56% (-0.15%) флэт
  - broadphase: 15.66% -> 10.84% (-4.82%) спад
  - nav_ai: 14.16% -> 2.85% (-11.31%) спад
  - inside_volatile: 12.01% -> 17.11% (+5.10%) РОСТ
  - fastutil: 8.54% -> 7.30% (-1.24%) спад
  - java_util: 7.01% -> 8.73% (+1.71%) РОСТ
  - paletted: 6.41% -> 5.44% (-0.97%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
