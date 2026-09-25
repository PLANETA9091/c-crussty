# absorb ROUND (a33-456w3, run 36119688267, branch round-456-anchor-33, head d4deb33)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7137973 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7137973 (поллов=5); TPS_exp=2.30; normalized=-4.4%
- GC: young=110, Full=10, total=24.2s, avg=202ms, max=2543ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117321 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.72% (-1.46%) спад
  - fluid: 16.72% -> 15.69% (-1.03%) спад
  - broadphase: 15.66% -> 15.25% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.45%) флэт
  - inside_volatile: 12.01% -> 11.24% (-0.76%) флэт
  - fastutil: 8.54% -> 8.27% (-0.27%) флэт
  - java_util: 7.01% -> 6.44% (-0.57%) флэт
  - paletted: 6.41% -> 6.24% (-0.17%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
