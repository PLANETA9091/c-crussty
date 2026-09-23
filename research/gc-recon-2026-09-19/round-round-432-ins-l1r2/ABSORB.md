# absorb ROUND (round-432-ins-l1r2, run 35891261495, branch round-432-ins-l1r2, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8736665 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 8736665 (поллов=5); TPS_exp=2.64; normalized=+17.5%
- GC: young=131, Full=10, total=19.9s, avg=141ms, max=2098ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102160 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.94% (+0.22%) флэт
  - broadphase: 15.66% -> 9.32% (-6.33%) спад
  - nav_ai: 14.16% -> 3.29% (-10.87%) спад
  - inside_volatile: 12.01% -> 13.91% (+1.90%) РОСТ
  - fastutil: 8.54% -> 6.23% (-2.31%) спад
  - java_util: 7.01% -> 8.39% (+1.37%) РОСТ
  - paletted: 6.41% -> 6.21% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
