# absorb ROUND (454-a5, run 36093432099, branch round-454-anchor-5, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6691686 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6691686 (поллов=6); TPS_exp=2.21; normalized=+6.4%
- GC: young=114, Full=10, total=23.4s, avg=189ms, max=2341ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116461 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.61% (-1.56%) спад
  - fluid: 16.72% -> 16.17% (-0.55%) флэт
  - broadphase: 15.66% -> 15.07% (-0.58%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.68%) флэт
  - inside_volatile: 12.01% -> 11.31% (-0.69%) флэт
  - fastutil: 8.54% -> 8.82% (+0.28%) флэт
  - java_util: 7.01% -> 6.27% (-0.74%) флэт
  - paletted: 6.41% -> 6.28% (-0.13%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
