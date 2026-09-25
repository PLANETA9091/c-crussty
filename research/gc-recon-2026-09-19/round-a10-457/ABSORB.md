# absorb ROUND (a10-457, run 36131678116, branch round-457-anchor-10, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8622159 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8622159 (поллов=5); TPS_exp=2.61; normalized=+3.3%
- GC: young=126, Full=10, total=22.1s, avg=163ms, max=2091ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113070 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.70% (-2.47%) спад
  - fluid: 16.72% -> 16.52% (-0.20%) флэт
  - broadphase: 15.66% -> 14.96% (-0.69%) флэт
  - nav_ai: 14.16% -> 13.64% (-0.52%) флэт
  - inside_volatile: 12.01% -> 10.71% (-1.30%) спад
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 7.22% (+0.21%) флэт
  - paletted: 6.41% -> 7.03% (+0.63%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
