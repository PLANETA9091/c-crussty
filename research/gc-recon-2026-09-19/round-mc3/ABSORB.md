# absorb ROUND (mc3, run 35740304790, branch round-415-a-mc3, head e1771f1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7159725 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=0.40 @ 7159725 (поллов=5); TPS_exp=2.31; normalized=-82.7%
- GC: young=71, Full=9, total=14.7s, avg=183ms, max=2099ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113305 сэмплов (базлайн 115655)
  - items: 31.17% -> 27.24% (-3.93%) спад
  - fluid: 16.72% -> 13.44% (-3.27%) спад
  - broadphase: 15.66% -> 8.76% (-6.90%) спад
  - nav_ai: 14.16% -> 3.20% (-10.96%) спад
  - inside_volatile: 12.01% -> 9.16% (-2.85%) спад
  - fastutil: 8.54% -> 4.61% (-3.93%) спад
  - java_util: 7.01% -> 6.25% (-0.76%) флэт
  - paletted: 6.41% -> 4.76% (-1.65%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
