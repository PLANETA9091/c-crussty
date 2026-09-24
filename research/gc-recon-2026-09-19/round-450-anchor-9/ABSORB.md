# absorb ROUND (450-anchor-9, run 36050608121, branch round-450-anchor-9, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7119774 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 7119774 (поллов=6); TPS_exp=2.30; normalized=-2.1%
- GC: young=105, Full=9, total=20.2s, avg=177ms, max=2452ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116242 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.67% (-1.50%) спад
  - fluid: 16.72% -> 15.59% (-1.13%) спад
  - broadphase: 15.66% -> 15.64% (-0.01%) флэт
  - nav_ai: 14.16% -> 14.11% (-0.06%) флэт
  - inside_volatile: 12.01% -> 11.59% (-0.42%) флэт
  - fastutil: 8.54% -> 9.38% (+0.84%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.26% (-0.15%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
