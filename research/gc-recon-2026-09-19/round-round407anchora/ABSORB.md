# absorb ROUND (round407anchora, run 35659184896, branch round-407-anchora, head d3e20bc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6604167 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6604167 (поллов=6); TPS_exp=2.19; normalized=+9.6%
- GC: young=108, Full=9, total=19.9s, avg=170ms, max=2441ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114654 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.71% (-0.46%) флэт
  - fluid: 16.72% -> 16.18% (-0.53%) флэт
  - broadphase: 15.66% -> 15.65% (-0.01%) флэт
  - nav_ai: 14.16% -> 14.34% (+0.18%) флэт
  - inside_volatile: 12.01% -> 11.92% (-0.09%) флэт
  - fastutil: 8.54% -> 8.87% (+0.33%) флэт
  - java_util: 7.01% -> 7.23% (+0.21%) флэт
  - paletted: 6.41% -> 6.54% (+0.14%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
