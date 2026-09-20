# absorb MEGA-ROUND-1 items_sweep (run 35539380853, head 622a3ef, branch round-396-c-items_sweep) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=9924645 (band OUT), lever=ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **PASS**
  - arm-marker: [crussty-plugin] items_sweep: net/minecraft/world/entity/item/ItemEntity armed, retransform rc=0
  - arm-marker: [crussty-plugin] items_sweep: net/minecraft/world/entity/item/ItemEntity armed, retransform rc=0
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.80 @ 9924645; TPS_exp(банк-интерполяция)=2.89; DELTA=-3.1% (БЫЛО 2.89 -> СТАЛО 2.80 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=125, Full=9, total=22.6s, avg=169ms, max=2552ms
- T5 ЛЕЙН-КАРТА: total=114609 сэмплов (базлайн 115655)
  - paletted: банк 6.41% -> лег 7.57% (+1.17%) [7408->8679] РОСТ
  - fluid: банк 16.72% -> лег 17.70% (+0.99%) [19332->20288] флэт
  - fastutil: банк 8.54% -> лег 8.79% (+0.25%) [9874->10074] флэт
  - inside_volatile: банк 12.01% -> лег 12.08% (+0.08%) [13885->13850] флэт
  - players_packets: банк 0.01% -> лег 0.01% (-0.00%) [12->9] флэт
  - java_util: банк 7.01% -> лег 6.99% (-0.02%) [8110->8009] флэт
  - nav_ai: банк 14.16% -> лег 13.67% (-0.50%) [16380->15663] флэт
  - broadphase: банк 15.66% -> лег 14.55% (-1.11%) [18108->16674] спад
  - items: банк 31.17% -> лег 29.61% (-1.56%) [36051->33935] спад

## VERDICT: **MEASURED**
