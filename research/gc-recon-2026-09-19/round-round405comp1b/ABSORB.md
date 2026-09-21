# absorb ROUND (round405comp1b, run 35633594662, branch round-405-comp-l1, head fef3746)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6964660 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6964660 (поллов=5); TPS_exp=2.27; normalized=+14.8%
- GC: young=104, Full=9, total=17.4s, avg=154ms, max=2180ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110669 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.30% (-0.42%) флэт
  - broadphase: 15.66% -> 14.63% (-1.03%) спад
  - nav_ai: 14.16% -> 10.72% (-3.44%) спад
  - inside_volatile: 12.01% -> 12.48% (+0.48%) флэт
  - fastutil: 8.54% -> 7.93% (-0.60%) флэт
  - java_util: 7.01% -> 7.44% (+0.43%) флэт
  - paletted: 6.41% -> 5.74% (-0.67%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
