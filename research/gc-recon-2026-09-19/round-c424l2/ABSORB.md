# absorb ROUND (c424l2, run 35827071370, branch round-424-c-l2, head 6f92ea7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6887410 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6887410 (поллов=6); TPS_exp=2.25; normalized=+20.0%
- GC: young=1129, Full=9, total=26.7s, avg=23ms, max=2286ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108322 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.10% (-0.62%) флэт
  - broadphase: 15.66% -> 14.93% (-0.72%) флэт
  - nav_ai: 14.16% -> 9.14% (-5.02%) спад
  - inside_volatile: 12.01% -> 12.63% (+0.62%) флэт
  - fastutil: 8.54% -> 7.58% (-0.95%) флэт
  - java_util: 7.01% -> 7.90% (+0.89%) флэт
  - paletted: 6.41% -> 5.55% (-0.86%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
