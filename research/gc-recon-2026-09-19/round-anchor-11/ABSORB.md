# absorb ROUND (anchor-11, run 36093483232, branch round-454-anchor-11, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9148288 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 9148288 (поллов=6); TPS_exp=2.73; normalized=-4.6%
- GC: young=117, Full=10, total=26.4s, avg=208ms, max=2770ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116971 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.04% (-1.13%) спад
  - fluid: 16.72% -> 18.44% (+1.73%) РОСТ
  - broadphase: 15.66% -> 14.44% (-1.21%) спад
  - nav_ai: 14.16% -> 13.46% (-0.70%) флэт
  - inside_volatile: 12.01% -> 12.22% (+0.22%) флэт
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 7.02% (+0.01%) флэт
  - paletted: 6.41% -> 7.67% (+1.27%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
