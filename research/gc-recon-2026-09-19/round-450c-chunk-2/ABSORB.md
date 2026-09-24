# absorb ROUND (450c-chunk-2, run 36053645579, branch round-450c-chunk-2, head 942d128)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6774396 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6774396 (поллов=6); TPS_exp=2.23; normalized=+16.8%
- GC: young=106, Full=9, total=20.0s, avg=174ms, max=3039ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103890 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.33% (-0.38%) флэт
  - broadphase: 15.66% -> 9.94% (-5.72%) спад
  - nav_ai: 14.16% -> 3.18% (-10.98%) спад
  - inside_volatile: 12.01% -> 16.24% (+4.24%) РОСТ
  - fastutil: 8.54% -> 6.71% (-1.82%) спад
  - java_util: 7.01% -> 8.45% (+1.43%) РОСТ
  - paletted: 6.41% -> 5.32% (-1.08%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
