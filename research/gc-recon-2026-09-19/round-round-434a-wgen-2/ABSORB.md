# absorb ROUND (round-434a-wgen-2, run 35917140499, branch round-434a-wgen-2, head 062baea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6456229 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6456229 (поллов=6); TPS_exp=2.16; normalized=-7.3%
- GC: young=500, Full=9, total=22.2s, avg=44ms, max=2715ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110879 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 13.86% (-2.85%) спад
  - broadphase: 15.66% -> 8.78% (-6.88%) спад
  - nav_ai: 14.16% -> 5.85% (-8.31%) спад
  - inside_volatile: 12.01% -> 14.19% (+2.18%) РОСТ
  - fastutil: 8.54% -> 7.07% (-1.46%) спад
  - java_util: 7.01% -> 7.91% (+0.90%) флэт
  - paletted: 6.41% -> 5.55% (-0.85%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
