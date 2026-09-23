# absorb ROUND (anchor422a, run 35809282117, branch round-422-mga, head 99ffefa)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6431132 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6431132 (поллов=5); TPS_exp=2.15; normalized=+20.8%
- GC: young=1132, Full=9, total=28.4s, avg=25ms, max=2404ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107626 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.55% (+0.83%) флэт
  - broadphase: 15.66% -> 14.05% (-1.60%) спад
  - nav_ai: 14.16% -> 9.37% (-4.79%) спад
  - inside_volatile: 12.01% -> 11.65% (-0.36%) флэт
  - fastutil: 8.54% -> 8.16% (-0.38%) флэт
  - java_util: 7.01% -> 7.82% (+0.81%) флэт
  - paletted: 6.41% -> 6.51% (+0.10%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
