# absorb ROUND (round-440c-chunk4-1, run 35951314959, branch round-440c-chunk4-1, head 8938afb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8708910 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 8708910 (поллов=5); TPS_exp=2.63; normalized=+10.1%
- GC: young=112, Full=9, total=22.7s, avg=187ms, max=3152ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105647 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.52% (+0.80%) флэт
  - broadphase: 15.66% -> 9.59% (-6.07%) спад
  - nav_ai: 14.16% -> 3.37% (-10.79%) спад
  - inside_volatile: 12.01% -> 16.30% (+4.30%) РОСТ
  - fastutil: 8.54% -> 6.18% (-2.35%) спад
  - java_util: 7.01% -> 8.94% (+1.93%) РОСТ
  - paletted: 6.41% -> 6.39% (-0.01%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
