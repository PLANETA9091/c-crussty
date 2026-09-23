# absorb ROUND (mega421a, run 35806660177, branch round-421-mga, head 99ffefa)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8820417 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8820417 (поллов=5); TPS_exp=2.66; normalized=+20.5%
- GC: young=1126, Full=8, total=28.1s, avg=25ms, max=2531ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109304 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.61% (-0.11%) флэт
  - broadphase: 15.66% -> 13.10% (-2.56%) спад
  - nav_ai: 14.16% -> 9.68% (-4.49%) спад
  - inside_volatile: 12.01% -> 12.72% (+0.71%) флэт
  - fastutil: 8.54% -> 8.29% (-0.25%) флэт
  - java_util: 7.01% -> 8.25% (+1.24%) РОСТ
  - paletted: 6.41% -> 6.42% (+0.02%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
