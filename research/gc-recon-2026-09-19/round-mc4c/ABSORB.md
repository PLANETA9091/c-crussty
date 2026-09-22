# absorb ROUND (mc4c, run 35788923353, branch round-420-mc4c, head 20c9fdc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7073496 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 7073496 (поллов=6); TPS_exp=2.29; normalized=+11.4%
- GC: young=104, Full=9, total=18.1s, avg=160ms, max=2313ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107716 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.00% (-0.72%) флэт
  - broadphase: 15.66% -> 10.17% (-5.49%) спад
  - nav_ai: 14.16% -> 4.05% (-10.11%) спад
  - inside_volatile: 12.01% -> 11.42% (-0.59%) флэт
  - fastutil: 8.54% -> 5.87% (-2.67%) спад
  - java_util: 7.01% -> 7.60% (+0.59%) флэт
  - paletted: 6.41% -> 5.31% (-1.10%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
