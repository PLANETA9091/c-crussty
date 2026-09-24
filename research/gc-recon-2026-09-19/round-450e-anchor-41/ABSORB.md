# absorb ROUND (450e-anchor-41, run 36066472050, branch round-450-anchor-41, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6715206 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6715206 (поллов=5); TPS_exp=2.21; normalized=+3.9%
- GC: young=112, Full=10, total=26.8s, avg=220ms, max=3139ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115841 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.41% (-1.76%) спад
  - fluid: 16.72% -> 16.59% (-0.13%) флэт
  - broadphase: 15.66% -> 14.59% (-1.07%) спад
  - nav_ai: 14.16% -> 13.37% (-0.79%) флэт
  - inside_volatile: 12.01% -> 11.01% (-1.00%) флэт
  - fastutil: 8.54% -> 8.45% (-0.08%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 6.76% (+0.35%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
