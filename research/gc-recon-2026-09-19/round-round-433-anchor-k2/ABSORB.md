# absorb ROUND (round-433-anchor-k2, run 35910383285, branch round-433-anchor-k2, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7104310 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7104310 (поллов=6); TPS_exp=2.29; normalized=+0.2%
- GC: young=108, Full=8, total=20.8s, avg=179ms, max=2432ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116251 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 15.60% (-1.12%) спад
  - broadphase: 15.66% -> 15.24% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.85% (-0.31%) флэт
  - inside_volatile: 12.01% -> 11.61% (-0.40%) флэт
  - fastutil: 8.54% -> 8.88% (+0.34%) флэт
  - java_util: 7.01% -> 6.47% (-0.55%) флэт
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
