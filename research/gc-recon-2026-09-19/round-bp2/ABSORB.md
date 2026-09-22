# absorb ROUND (bp2, run 35723457526, branch round-413-bp2, head 125c474)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6788478 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.05 @ 6788478 (поллов=6); TPS_exp=2.23; normalized=-52.9%
- GC: young=64, Full=8, total=11.7s, avg=163ms, max=1901ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108291 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 12.41% (-4.31%) спад
  - broadphase: 15.66% -> 7.10% (-8.56%) спад
  - nav_ai: 14.16% -> 3.03% (-11.13%) спад
  - inside_volatile: 12.01% -> 8.29% (-3.72%) спад
  - fastutil: 8.54% -> 4.62% (-3.92%) спад
  - java_util: 7.01% -> 6.00% (-1.01%) спад
  - paletted: 6.41% -> 4.67% (-1.74%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
