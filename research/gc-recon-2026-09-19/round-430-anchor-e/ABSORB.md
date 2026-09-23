# absorb ROUND (430-anchor-e, run 35873198595, branch round-430-anchor-e, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8754557 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8754557 (поллов=5); TPS_exp=2.64; normalized=-9.2%
- GC: young=117, Full=10, total=27.1s, avg=213ms, max=2807ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117026 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.90% (-1.27%) спад
  - fluid: 16.72% -> 17.80% (+1.09%) РОСТ
  - broadphase: 15.66% -> 15.11% (-0.55%) флэт
  - nav_ai: 14.16% -> 13.68% (-0.49%) флэт
  - inside_volatile: 12.01% -> 11.97% (-0.04%) флэт
  - fastutil: 8.54% -> 8.83% (+0.29%) флэт
  - java_util: 7.01% -> 7.04% (+0.03%) флэт
  - paletted: 6.41% -> 7.46% (+1.06%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
