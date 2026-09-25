# absorb ROUND (451b-anchor-18, run 36073632084, branch round-451-anchor-18, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6417731 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6417731 (поллов=5); TPS_exp=2.15; normalized=-2.3%
- GC: young=110, Full=9, total=20.9s, avg=176ms, max=2443ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116504 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.30% (-1.87%) спад
  - fluid: 16.72% -> 15.67% (-1.04%) спад
  - broadphase: 15.66% -> 14.59% (-1.06%) спад
  - nav_ai: 14.16% -> 14.07% (-0.10%) флэт
  - inside_volatile: 12.01% -> 11.13% (-0.87%) флэт
  - fastutil: 8.54% -> 8.64% (+0.10%) флэт
  - java_util: 7.01% -> 6.39% (-0.63%) флэт
  - paletted: 6.41% -> 6.29% (-0.12%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
