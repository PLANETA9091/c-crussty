# absorb ROUND (round-434-cce-l3r4r, run 35914703293, branch round-434-cce-l3r4r, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7071799 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7071799 (поллов=6); TPS_exp=2.29; normalized=-3.9%
- GC: young=103, Full=9, total=20.6s, avg=184ms, max=2648ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104051 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.28% (-0.43%) флэт
  - broadphase: 15.66% -> 9.85% (-5.80%) спад
  - nav_ai: 14.16% -> 3.13% (-11.03%) спад
  - inside_volatile: 12.01% -> 15.75% (+3.74%) РОСТ
  - fastutil: 8.54% -> 6.45% (-2.09%) спад
  - java_util: 7.01% -> 8.76% (+1.75%) РОСТ
  - paletted: 6.41% -> 5.44% (-0.96%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
