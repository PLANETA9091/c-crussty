# absorb ROUND (a7, run 36104762869, branch round-455-anchor-7, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6720243 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6720243 (поллов=5); TPS_exp=2.21; normalized=-0.6%
- GC: young=111, Full=9, total=21.0s, avg=175ms, max=2376ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116458 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.34% (-1.83%) спад
  - fluid: 16.72% -> 15.48% (-1.23%) спад
  - broadphase: 15.66% -> 14.96% (-0.70%) флэт
  - nav_ai: 14.16% -> 13.75% (-0.42%) флэт
  - inside_volatile: 12.01% -> 11.42% (-0.58%) флэт
  - fastutil: 8.54% -> 8.19% (-0.35%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
