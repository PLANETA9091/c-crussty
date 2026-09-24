# absorb ROUND (round-435-w3-1, run 35924481185, branch round-435-w3-1, head 062baea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8878218 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8878218 (поллов=6); TPS_exp=2.67; normalized=-2.6%
- GC: young=758, Full=10, total=24.4s, avg=32ms, max=2426ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107770 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.45% (-2.27%) спад
  - broadphase: 15.66% -> 8.21% (-7.44%) спад
  - nav_ai: 14.16% -> 6.00% (-8.16%) спад
  - inside_volatile: 12.01% -> 13.44% (+1.44%) РОСТ
  - fastutil: 8.54% -> 7.36% (-1.17%) спад
  - java_util: 7.01% -> 7.84% (+0.82%) флэт
  - paletted: 6.41% -> 6.43% (+0.03%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
