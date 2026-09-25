# absorb ROUND (diet454-10, run 36100433356, branch round-454c-diet-10, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6698631 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6698631 (поллов=5); TPS_exp=2.21; normalized=+4.1%
- GC: young=100, Full=9, total=18.8s, avg=173ms, max=2614ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104052 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.22% (-0.50%) флэт
  - broadphase: 15.66% -> 9.72% (-5.94%) спад
  - nav_ai: 14.16% -> 3.23% (-10.94%) спад
  - inside_volatile: 12.01% -> 17.08% (+5.07%) РОСТ
  - fastutil: 8.54% -> 6.50% (-2.04%) спад
  - java_util: 7.01% -> 8.51% (+1.49%) РОСТ
  - paletted: 6.41% -> 5.18% (-1.22%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
