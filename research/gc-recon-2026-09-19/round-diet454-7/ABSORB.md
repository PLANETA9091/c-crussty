# absorb ROUND (diet454-7, run 36098955646, branch round-454c-diet-7, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6470217 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6470217 (поллов=5); TPS_exp=2.16; normalized=+15.7%
- GC: young=105, Full=9, total=21.9s, avg=192ms, max=2783ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104593 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.99% (+0.27%) флэт
  - broadphase: 15.66% -> 9.41% (-6.25%) спад
  - nav_ai: 14.16% -> 3.06% (-11.11%) спад
  - inside_volatile: 12.01% -> 16.27% (+4.27%) РОСТ
  - fastutil: 8.54% -> 6.45% (-2.08%) спад
  - java_util: 7.01% -> 8.31% (+1.30%) РОСТ
  - paletted: 6.41% -> 6.09% (-0.32%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
