# absorb ROUND (round405scd4, run 35625905853, branch round-405-scd4, head 60fe902)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7115766 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 7115766 (поллов=5); TPS_exp=2.30; normalized=+34.9%
- GC: young=123, Full=9, total=22.3s, avg=169ms, max=2561ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113215 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 19.00% (+2.29%) РОСТ
  - broadphase: 15.66% -> 14.15% (-1.51%) спад
  - nav_ai: 14.16% -> 10.51% (-3.65%) спад
  - inside_volatile: 12.01% -> 12.65% (+0.65%) флэт
  - fastutil: 8.54% -> 7.77% (-0.76%) флэт
  - java_util: 7.01% -> 6.85% (-0.17%) флэт
  - paletted: 6.41% -> 6.46% (+0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
