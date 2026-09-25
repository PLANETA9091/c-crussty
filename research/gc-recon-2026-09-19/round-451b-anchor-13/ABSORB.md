# absorb ROUND (451b-anchor-13, run 36073584790, branch round-451-anchor-13, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6762251 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 6762251 (поллов=5); TPS_exp=2.22; normalized=-14.5%
- GC: young=112, Full=9, total=23.4s, avg=193ms, max=2564ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115857 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.95% (-2.22%) спад
  - fluid: 16.72% -> 16.76% (+0.05%) флэт
  - broadphase: 15.66% -> 15.41% (-0.25%) флэт
  - nav_ai: 14.16% -> 13.47% (-0.69%) флэт
  - inside_volatile: 12.01% -> 10.59% (-1.42%) спад
  - fastutil: 8.54% -> 8.73% (+0.20%) флэт
  - java_util: 7.01% -> 6.19% (-0.83%) флэт
  - paletted: 6.41% -> 6.91% (+0.50%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
