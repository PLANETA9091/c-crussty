# absorb ROUND (a6-457, run 36131626724, branch round-457-anchor-6, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8701424 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 8701424 (поллов=5); TPS_exp=2.63; normalized=-12.6%
- GC: young=114, Full=9, total=23.9s, avg=195ms, max=2825ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117059 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.66% (-1.51%) спад
  - fluid: 16.72% -> 17.80% (+1.08%) РОСТ
  - broadphase: 15.66% -> 14.97% (-0.68%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.50%) флэт
  - inside_volatile: 12.01% -> 11.37% (-0.64%) флэт
  - fastutil: 8.54% -> 8.40% (-0.14%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 7.39% (+0.99%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
