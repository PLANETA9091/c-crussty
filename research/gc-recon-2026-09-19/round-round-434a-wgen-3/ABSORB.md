# absorb ROUND (round-434a-wgen-3, run 35917154890, branch round-434a-wgen-3, head 062baea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6977107 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.80 @ 6977107 (поллов=5); TPS_exp=2.27; normalized=-20.6%
- GC: young=501, Full=10, total=24.8s, avg=49ms, max=2692ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110504 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 13.94% (-2.77%) спад
  - broadphase: 15.66% -> 8.74% (-6.91%) спад
  - nav_ai: 14.16% -> 6.07% (-8.09%) спад
  - inside_volatile: 12.01% -> 14.29% (+2.29%) РОСТ
  - fastutil: 8.54% -> 7.38% (-1.16%) спад
  - java_util: 7.01% -> 7.85% (+0.84%) флэт
  - paletted: 6.41% -> 5.33% (-1.08%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
