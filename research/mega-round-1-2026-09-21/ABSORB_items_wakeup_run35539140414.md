# absorb MEGA-ROUND-1 items_wakeup (run 35539140414, head f01cd1d, branch round-396-i-items_wakeup) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=8529571 (band OK), lever=ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **PASS**
  - arm-marker: [crussty-plugin] items_wakeup: net/minecraft/world/entity/item/ItemEntity armed, retransform rc=0
  - arm-marker: [crussty-plugin] items_wakeup: net/minecraft/world/entity/item/ItemEntity armed, retransform rc=0
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.70 @ 8529571; TPS_exp(банк-интерполяция)=2.60; DELTA=+4.0% (БЫЛО 2.60 -> СТАЛО 2.70 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=126, Full=9, total=21.0s, avg=156ms, max=2135ms
- T5 ЛЕЙН-КАРТА: total=110996 сэмплов (базлайн 115655)
  - paletted: банк 6.41% -> лег 6.92% (+0.52%) [7408->7686] флэт
  - fluid: банк 16.72% -> лег 16.79% (+0.08%) [19332->18638] флэт
  - players_packets: банк 0.01% -> лег 0.00% (-0.01%) [12->4] флэт
  - fastutil: банк 8.54% -> лег 8.51% (-0.03%) [9874->9448] флэт
  - nav_ai: банк 14.16% -> лег 14.05% (-0.11%) [16380->15600] флэт
  - broadphase: банк 15.66% -> лег 15.50% (-0.16%) [18108->17205] флэт
  - java_util: банк 7.01% -> лег 6.61% (-0.40%) [8110->7342] флэт
  - inside_volatile: банк 12.01% -> лег 11.10% (-0.90%) [13885->12324] флэт
  - items: банк 31.17% -> лег 28.61% (-2.56%) [36051->31761] спад

## VERDICT: **MEASURED**
