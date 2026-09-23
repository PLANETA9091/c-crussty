# absorb ROUND (round-434-mega-m3r1, run 35914813696, branch round-434-mega-m3r1, head b15f329)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7100628 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7100628 (поллов=5); TPS_exp=2.29; normalized=+13.3%
- GC: young=108, Full=9, total=19.9s, avg=170ms, max=2914ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104719 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.17% (+0.46%) флэт
  - broadphase: 15.66% -> 10.01% (-5.65%) спад
  - nav_ai: 14.16% -> 3.30% (-10.87%) спад
  - inside_volatile: 12.01% -> 13.39% (+1.39%) РОСТ
  - fastutil: 8.54% -> 6.20% (-2.34%) спад
  - java_util: 7.01% -> 7.24% (+0.22%) флэт
  - paletted: 6.41% -> 5.83% (-0.58%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
