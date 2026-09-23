# absorb ROUND (430a-anchor-a2, run 35876671116, branch round-430a-anchor-a2, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6927234 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6927234 (поллов=5); TPS_exp=2.26; normalized=-2.6%
- GC: young=109, Full=9, total=20.9s, avg=177ms, max=2724ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116896 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.21% (-1.96%) спад
  - fluid: 16.72% -> 15.96% (-0.75%) флэт
  - broadphase: 15.66% -> 15.22% (-0.43%) флэт
  - nav_ai: 14.16% -> 14.06% (-0.11%) флэт
  - inside_volatile: 12.01% -> 11.26% (-0.75%) флэт
  - fastutil: 8.54% -> 8.79% (+0.25%) флэт
  - java_util: 7.01% -> 8.03% (+1.01%) РОСТ
  - paletted: 6.41% -> 6.31% (-0.10%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
