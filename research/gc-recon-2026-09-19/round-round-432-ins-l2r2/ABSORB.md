# absorb ROUND (round-432-ins-l2r2, run 35891316506, branch round-432-ins-l2r2, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7114159 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=2.70 @ 7114159 (поллов=5); TPS_exp=2.30; normalized=+17.5%
- GC: young=110, Full=9, total=19.4s, avg=163ms, max=2615ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105249 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.51% (-0.20%) флэт
  - broadphase: 15.66% -> 9.75% (-5.91%) спад
  - nav_ai: 14.16% -> 3.26% (-10.90%) спад
  - inside_volatile: 12.01% -> 15.44% (+3.44%) РОСТ
  - fastutil: 8.54% -> 6.58% (-1.96%) спад
  - java_util: 7.01% -> 8.10% (+1.09%) РОСТ
  - paletted: 6.41% -> 5.65% (-0.76%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **CRASH-REFUTED**
