# absorb ROUND (a32, run 36108860521, branch round-455-anchor-32, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6791671 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6791671 (поллов=6); TPS_exp=2.23; normalized=-1.3%
- GC: young=108, Full=9, total=21.1s, avg=181ms, max=2464ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115752 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.96% (-2.21%) спад
  - fluid: 16.72% -> 15.63% (-1.09%) спад
  - broadphase: 15.66% -> 15.67% (+0.01%) флэт
  - nav_ai: 14.16% -> 14.02% (-0.15%) флэт
  - inside_volatile: 12.01% -> 10.79% (-1.21%) спад
  - fastutil: 8.54% -> 8.54% (-0.00%) флэт
  - java_util: 7.01% -> 6.64% (-0.37%) флэт
  - paletted: 6.41% -> 6.03% (-0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
