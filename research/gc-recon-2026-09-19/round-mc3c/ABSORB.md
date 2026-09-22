# absorb ROUND (mc3c, run 35765678598, branch round-417-a-mc3c, head 20c9fdc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8697388 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8697388 (поллов=5); TPS_exp=2.63; normalized=+21.6%
- GC: young=121, Full=10, total=19.7s, avg=151ms, max=2061ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105334 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.30% (-0.42%) флэт
  - broadphase: 15.66% -> 9.13% (-6.53%) спад
  - nav_ai: 14.16% -> 4.37% (-9.79%) спад
  - inside_volatile: 12.01% -> 10.95% (-1.06%) спад
  - fastutil: 8.54% -> 6.19% (-2.35%) спад
  - java_util: 7.01% -> 8.08% (+1.07%) РОСТ
  - paletted: 6.41% -> 6.15% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
