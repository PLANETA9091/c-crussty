# absorb ROUND (round-438-pd-rev1, run 35941692866, branch round-438-pd-rev1, head d8c8463)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7031449 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7031449 (поллов=6); TPS_exp=2.28; normalized=+0.9%
- GC: young=115, Full=9, total=20.9s, avg=168ms, max=2411ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117252 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.71% (-1.46%) спад
  - fluid: 16.72% -> 15.72% (-0.99%) флэт
  - broadphase: 15.66% -> 14.88% (-0.78%) флэт
  - nav_ai: 14.16% -> 13.73% (-0.43%) флэт
  - inside_volatile: 12.01% -> 11.77% (-0.24%) флэт
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 6.39% (-0.63%) флэт
  - paletted: 6.41% -> 6.03% (-0.37%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
