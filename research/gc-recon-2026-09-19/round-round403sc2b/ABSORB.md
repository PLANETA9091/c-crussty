# absorb ROUND (round403sc2b, run 35615076298, branch round-403-sc2b, head d93894b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6705897 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6705897 (поллов=5); TPS_exp=2.21; normalized=+8.5%
- GC: young=111, Full=9, total=20.5s, avg=170ms, max=2407ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113028 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.90% (+0.19%) флэт
  - broadphase: 15.66% -> 13.89% (-1.76%) спад
  - nav_ai: 14.16% -> 11.60% (-2.56%) спад
  - inside_volatile: 12.01% -> 12.21% (+0.21%) флэт
  - fastutil: 8.54% -> 7.96% (-0.57%) флэт
  - java_util: 7.01% -> 7.33% (+0.32%) флэт
  - paletted: 6.41% -> 6.66% (+0.25%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
