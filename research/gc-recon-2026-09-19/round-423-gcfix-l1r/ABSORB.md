# absorb ROUND (round-423-gcfix-l1r, run 35822264943, branch round-423-a-gcfix-l1r, head 831394a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6982760 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6982760 (поллов=6); TPS_exp=2.27; normalized=+8.0%
- GC: young=4111, Full=11, total=43.8s, avg=11ms, max=2604ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110431 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.58% (-2.13%) спад
  - broadphase: 15.66% -> 15.07% (-0.59%) флэт
  - nav_ai: 14.16% -> 5.70% (-8.46%) спад
  - inside_volatile: 12.01% -> 11.89% (-0.12%) флэт
  - fastutil: 8.54% -> 9.07% (+0.53%) флэт
  - java_util: 7.01% -> 7.88% (+0.87%) флэт
  - paletted: 6.41% -> 5.64% (-0.77%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
