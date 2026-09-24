# absorb ROUND (round-440-pd-1, run 35950542793, branch round-440-pd-1, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7030461 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7030461 (поллов=6); TPS_exp=2.28; normalized=-3.5%
- GC: young=110, Full=9, total=21.4s, avg=179ms, max=2482ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116536 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.00% (-1.17%) спад
  - fluid: 16.72% -> 16.16% (-0.56%) флэт
  - broadphase: 15.66% -> 15.14% (-0.52%) флэт
  - nav_ai: 14.16% -> 13.68% (-0.48%) флэт
  - inside_volatile: 12.01% -> 11.39% (-0.61%) флэт
  - fastutil: 8.54% -> 8.48% (-0.05%) флэт
  - java_util: 7.01% -> 6.54% (-0.48%) флэт
  - paletted: 6.41% -> 6.89% (+0.49%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
