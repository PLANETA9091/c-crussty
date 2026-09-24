# absorb ROUND (round-442-anchor-13, run 35957185944, branch round-442-anchor-13, head 335f170)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7106711 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7106711 (поллов=5); TPS_exp=2.30; normalized=-4.2%
- GC: young=107, Full=9, total=20.5s, avg=177ms, max=2441ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116983 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.01% (-1.16%) спад
  - fluid: 16.72% -> 15.81% (-0.91%) флэт
  - broadphase: 15.66% -> 15.16% (-0.50%) флэт
  - nav_ai: 14.16% -> 13.36% (-0.81%) флэт
  - inside_volatile: 12.01% -> 11.55% (-0.46%) флэт
  - fastutil: 8.54% -> 8.38% (-0.16%) флэт
  - java_util: 7.01% -> 6.50% (-0.51%) флэт
  - paletted: 6.41% -> 6.05% (-0.35%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
