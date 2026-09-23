# absorb ROUND (round-434-mega-m2r2, run 35919295421, branch round-434-mega-m2r2, head b15f329)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6982878 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6982878 (поллов=5); TPS_exp=2.27; normalized=+10.2%
- GC: young=104, Full=9, total=20.0s, avg=177ms, max=2751ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103702 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.70% (-0.02%) флэт
  - broadphase: 15.66% -> 10.26% (-5.39%) спад
  - nav_ai: 14.16% -> 3.33% (-10.83%) спад
  - inside_volatile: 12.01% -> 12.65% (+0.65%) флэт
  - fastutil: 8.54% -> 6.51% (-2.03%) спад
  - java_util: 7.01% -> 7.62% (+0.60%) флэт
  - paletted: 6.41% -> 5.77% (-0.63%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
