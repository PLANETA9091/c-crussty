# absorb ROUND (round406dleg6, run 35663105153, branch round-406-d-l6, head dac1140)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6585981 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 6585981 (поллов=5); TPS_exp=2.19; normalized=+32.7%
- GC: young=107, Full=9, total=18.3s, avg=158ms, max=2231ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109631 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.35% (-0.36%) флэт
  - broadphase: 15.66% -> 14.11% (-1.55%) спад
  - nav_ai: 14.16% -> 9.96% (-4.21%) спад
  - inside_volatile: 12.01% -> 12.48% (+0.47%) флэт
  - fastutil: 8.54% -> 7.85% (-0.69%) флэт
  - java_util: 7.01% -> 7.61% (+0.60%) флэт
  - paletted: 6.41% -> 5.64% (-0.76%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
