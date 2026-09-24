# absorb ROUND (450c-anchor-24, run 36058433747, branch round-450-anchor-24, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6972610 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6972610 (поллов=6); TPS_exp=2.27; normalized=-7.4%
- GC: young=107, Full=10, total=24.4s, avg=209ms, max=2515ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116088 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 15.68% (-1.04%) спад
  - broadphase: 15.66% -> 15.54% (-0.11%) флэт
  - nav_ai: 14.16% -> 14.46% (+0.29%) флэт
  - inside_volatile: 12.01% -> 11.39% (-0.62%) флэт
  - fastutil: 8.54% -> 8.73% (+0.19%) флэт
  - java_util: 7.01% -> 6.84% (-0.17%) флэт
  - paletted: 6.41% -> 5.98% (-0.43%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
