# absorb ROUND (round-423-gcfix-l2, run 35822165157, branch round-423-a-gcfix-l2, head 831394a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6708244 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6708244 (поллов=6); TPS_exp=2.21; normalized=+6.3%
- GC: young=4136, Full=11, total=46.6s, avg=11ms, max=2586ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110278 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.34% (-1.37%) спад
  - broadphase: 15.66% -> 15.22% (-0.44%) флэт
  - nav_ai: 14.16% -> 5.29% (-8.87%) спад
  - inside_volatile: 12.01% -> 12.24% (+0.24%) флэт
  - fastutil: 8.54% -> 9.33% (+0.79%) флэт
  - java_util: 7.01% -> 7.76% (+0.75%) флэт
  - paletted: 6.41% -> 5.69% (-0.72%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
