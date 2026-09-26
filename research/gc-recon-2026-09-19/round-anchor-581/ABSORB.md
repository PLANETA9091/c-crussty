# absorb ROUND (anchor-581, run 36206967778, branch round-463-anchor-581, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=10172621 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 10172621 (поллов=5); TPS_exp=2.94; normalized=-15.0%
- GC: young=122, Full=10, total=25.1s, avg=190ms, max=2670ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116643 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.18% (-1.99%) спад
  - fluid: 16.72% -> 18.18% (+1.47%) РОСТ
  - broadphase: 15.66% -> 14.46% (-1.19%) спад
  - nav_ai: 14.16% -> 13.51% (-0.65%) флэт
  - inside_volatile: 12.01% -> 11.83% (-0.18%) флэт
  - fastutil: 8.54% -> 8.68% (+0.14%) флэт
  - java_util: 7.01% -> 6.69% (-0.32%) флэт
  - paletted: 6.41% -> 7.58% (+1.18%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
