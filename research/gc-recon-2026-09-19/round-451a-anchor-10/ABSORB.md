# absorb ROUND (451a-anchor-10, run 36069521105, branch round-451-anchor-10, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6814103 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6814103 (поллов=6); TPS_exp=2.23; normalized=+9.7%
- GC: young=109, Full=8, total=20.9s, avg=179ms, max=2399ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116979 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.34% (-1.84%) спад
  - fluid: 16.72% -> 15.92% (-0.79%) флэт
  - broadphase: 15.66% -> 15.44% (-0.21%) флэт
  - nav_ai: 14.16% -> 14.22% (+0.05%) флэт
  - inside_volatile: 12.01% -> 11.48% (-0.52%) флэт
  - fastutil: 8.54% -> 9.58% (+1.04%) РОСТ
  - java_util: 7.01% -> 6.66% (-0.36%) флэт
  - paletted: 6.41% -> 6.31% (-0.10%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
