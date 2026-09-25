# absorb ROUND (chk454-1, run 36094570670, branch round-454b-chunk-1, head d6e77f4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7051356 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7051356 (поллов=5); TPS_exp=2.28; normalized=+5.1%
- GC: young=105, Full=9, total=19.8s, avg=174ms, max=2513ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105155 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.49% (-1.23%) спад
  - broadphase: 15.66% -> 9.67% (-5.98%) спад
  - nav_ai: 14.16% -> 3.21% (-10.96%) спад
  - inside_volatile: 12.01% -> 16.52% (+4.51%) РОСТ
  - fastutil: 8.54% -> 6.49% (-2.05%) спад
  - java_util: 7.01% -> 8.40% (+1.38%) РОСТ
  - paletted: 6.41% -> 4.97% (-1.44%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
