# absorb ROUND (431c-ins-l3r, run 35887020334, branch round-431c-ins-l3r, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6790672 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6790672 (поллов=5); TPS_exp=2.23; normalized=+16.6%
- GC: young=104, Full=9, total=20.8s, avg=184ms, max=2718ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103796 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.70% (+0.99%) флэт
  - broadphase: 15.66% -> 9.72% (-5.94%) спад
  - nav_ai: 14.16% -> 3.16% (-11.00%) спад
  - inside_volatile: 12.01% -> 15.00% (+3.00%) РОСТ
  - fastutil: 8.54% -> 6.68% (-1.86%) спад
  - java_util: 7.01% -> 8.56% (+1.55%) РОСТ
  - paletted: 6.41% -> 6.31% (-0.09%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
