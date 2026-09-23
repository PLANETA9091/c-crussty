# absorb ROUND (round-436c-anchor-2, run 35930840311, branch round-436c-anchor-2, head e05994c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9244053 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 9244053 (поллов=6); TPS_exp=2.75; normalized=+0.2%
- GC: young=114, Full=10, total=23.7s, avg=191ms, max=2608ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116691 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.48% (-1.70%) спад
  - fluid: 16.72% -> 17.94% (+1.23%) РОСТ
  - broadphase: 15.66% -> 14.88% (-0.78%) флэт
  - nav_ai: 14.16% -> 13.41% (-0.75%) флэт
  - inside_volatile: 12.01% -> 11.55% (-0.46%) флэт
  - fastutil: 8.54% -> 9.00% (+0.46%) флэт
  - java_util: 7.01% -> 6.96% (-0.05%) флэт
  - paletted: 6.41% -> 7.68% (+1.27%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
