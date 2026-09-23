# absorb ROUND (round-433-cce-l3r5, run 35910356313, branch round-433-cce-l3r5, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8830427 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 8830427 (поллов=5); TPS_exp=2.66; normalized=+16.6%
- GC: young=122, Full=9, total=18.2s, avg=139ms, max=2031ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=101795 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.16% (-0.55%) флэт
  - broadphase: 15.66% -> 8.64% (-7.02%) спад
  - nav_ai: 14.16% -> 3.29% (-10.87%) спад
  - inside_volatile: 12.01% -> 15.19% (+3.18%) РОСТ
  - fastutil: 8.54% -> 6.59% (-1.94%) спад
  - java_util: 7.01% -> 8.88% (+1.87%) РОСТ
  - paletted: 6.41% -> 6.12% (-0.29%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
