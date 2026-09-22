# absorb ROUND (colpusha, run 35781330371, branch round-419-a-cpa, head dc4afd9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=3804, col=PARALLEL, runner=6309799 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.65 @ 6309799 (поллов=6); TPS_exp=2.13; normalized=-22.4%
- GC: young=663, Full=9, total=48.5s, avg=72ms, max=2096ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111676 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 12.75% (-3.96%) спад
  - broadphase: 15.66% -> 12.35% (-3.31%) спад
  - nav_ai: 14.16% -> 15.08% (+0.92%) флэт
  - inside_volatile: 12.01% -> 9.99% (-2.01%) спад
  - fastutil: 8.54% -> 6.41% (-2.12%) спад
  - java_util: 7.01% -> 6.67% (-0.34%) флэт
  - paletted: 6.41% -> 4.39% (-2.02%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **DELIVERY-FAIL**
