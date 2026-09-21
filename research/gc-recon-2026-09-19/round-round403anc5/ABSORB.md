# absorb ROUND (round403anc5, run 35604266618, branch round-403-anchor3, head f19d5f5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6621660 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6621660 (поллов=6); TPS_exp=2.19; normalized=+2.6%
- GC: young=105, Full=7, total=19.1s, avg=171ms, max=2621ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113958 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.30% (-0.87%) флэт
  - fluid: 16.72% -> 16.02% (-0.70%) флэт
  - broadphase: 15.66% -> 15.83% (+0.18%) флэт
  - nav_ai: 14.16% -> 14.20% (+0.04%) флэт
  - inside_volatile: 12.01% -> 11.18% (-0.83%) флэт
  - fastutil: 8.54% -> 9.02% (+0.49%) флэт
  - java_util: 7.01% -> 6.88% (-0.14%) флэт
  - paletted: 6.41% -> 6.38% (-0.03%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
