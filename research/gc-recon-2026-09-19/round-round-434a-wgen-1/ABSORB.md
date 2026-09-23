# absorb ROUND (round-434a-wgen-1, run 35917113288, branch round-434a-wgen-1, head 062baea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6907564 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.70 @ 6907564 (поллов=5); TPS_exp=2.25; normalized=-24.6%
- GC: young=450, Full=9, total=23.2s, avg=51ms, max=2831ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110165 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 13.84% (-2.88%) спад
  - broadphase: 15.66% -> 8.55% (-7.11%) спад
  - nav_ai: 14.16% -> 5.68% (-8.49%) спад
  - inside_volatile: 12.01% -> 14.30% (+2.29%) РОСТ
  - fastutil: 8.54% -> 7.07% (-1.47%) спад
  - java_util: 7.01% -> 8.13% (+1.12%) РОСТ
  - paletted: 6.41% -> 5.46% (-0.94%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
