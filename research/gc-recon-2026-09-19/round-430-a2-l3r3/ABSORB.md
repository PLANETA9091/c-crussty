# absorb ROUND (430-a2-l3r3, run 35873184162, branch round-430-a2-l3r3, head 18151a9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6959768 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6959768 (поллов=5); TPS_exp=2.26; normalized=+10.4%
- GC: young=104, Full=9, total=19.3s, avg=170ms, max=2482ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103610 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.81% (+0.09%) флэт
  - broadphase: 15.66% -> 10.27% (-5.39%) спад
  - nav_ai: 14.16% -> 3.39% (-10.78%) спад
  - inside_volatile: 12.01% -> 12.77% (+0.76%) флэт
  - fastutil: 8.54% -> 7.07% (-1.46%) спад
  - java_util: 7.01% -> 7.50% (+0.49%) флэт
  - paletted: 6.41% -> 5.63% (-0.77%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
