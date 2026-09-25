# absorb ROUND (poi456-3, run 36126727936, branch round-456b-poi-3, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7181470 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7181470 (поллов=5); TPS_exp=2.31; normalized=-0.5%
- GC: young=102, Full=9, total=19.7s, avg=177ms, max=2842ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104856 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.31% (-0.41%) флэт
  - broadphase: 15.66% -> 9.85% (-5.81%) спад
  - nav_ai: 14.16% -> 3.15% (-11.01%) спад
  - inside_volatile: 12.01% -> 16.49% (+4.48%) РОСТ
  - fastutil: 8.54% -> 6.39% (-2.15%) спад
  - java_util: 7.01% -> 8.80% (+1.79%) РОСТ
  - paletted: 6.41% -> 5.18% (-1.23%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
