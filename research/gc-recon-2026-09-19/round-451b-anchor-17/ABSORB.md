# absorb ROUND (451b-anchor-17, run 36073621739, branch round-451-anchor-17, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6237460 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6237460 (поллов=5); TPS_exp=2.11; normalized=+4.1%
- GC: young=113, Full=10, total=26.1s, avg=212ms, max=2541ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116304 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.32% (-1.85%) спад
  - fluid: 16.72% -> 16.39% (-0.33%) флэт
  - broadphase: 15.66% -> 14.93% (-0.73%) флэт
  - nav_ai: 14.16% -> 13.43% (-0.73%) флэт
  - inside_volatile: 12.01% -> 10.80% (-1.21%) спад
  - fastutil: 8.54% -> 8.49% (-0.05%) флэт
  - java_util: 7.01% -> 6.35% (-0.66%) флэт
  - paletted: 6.41% -> 6.87% (+0.47%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
