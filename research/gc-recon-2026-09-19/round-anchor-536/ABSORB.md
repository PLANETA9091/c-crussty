# absorb ROUND (anchor-536, run 36206683361, branch round-463-anchor-536, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8712040 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 8712040 (поллов=5); TPS_exp=2.63; normalized=-12.7%
- GC: young=121, Full=10, total=22.9s, avg=175ms, max=2177ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112818 сэмплов (базлайн 115655)
  - items: 31.17% -> 27.95% (-3.23%) спад
  - fluid: 16.72% -> 16.24% (-0.47%) флэт
  - broadphase: 15.66% -> 15.20% (-0.46%) флэт
  - nav_ai: 14.16% -> 14.07% (-0.09%) флэт
  - inside_volatile: 12.01% -> 10.63% (-1.38%) спад
  - fastutil: 8.54% -> 9.17% (+0.64%) флэт
  - java_util: 7.01% -> 6.55% (-0.46%) флэт
  - paletted: 6.41% -> 6.96% (+0.56%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
