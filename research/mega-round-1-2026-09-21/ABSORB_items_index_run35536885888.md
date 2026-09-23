# absorb MEGA-ROUND-1 items_index (run 35536885888, head 3c068f2, branch round-396-a-items_index) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=6825137 (band OK), lever=ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **PASS**
  - arm-marker: [crussty-plugin] items_index: ARMED, retransform rc ItemEntity=0
  - arm-marker: [crussty-plugin] items_index: ARMED, retransform rc ItemEntity=0
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.30 @ 6825137; TPS_exp(банк-интерполяция)=2.24; DELTA=+2.9% (БЫЛО 2.24 -> СТАЛО 2.30 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=110, Full=9, total=21.8s, avg=183ms, max=2714ms
- T5 ЛЕЙН-КАРТА: total=114964 сэмплов (базлайн 115655)
  - fastutil: банк 8.54% -> лег 9.23% (+0.69%) [9874->10609] флэт
  - java_util: банк 7.01% -> лег 7.30% (+0.29%) [8110->8391] флэт
  - broadphase: банк 15.66% -> лег 15.73% (+0.07%) [18108->18085] флэт
  - players_packets: банк 0.01% -> лег 0.01% (-0.01%) [12->6] флэт
  - paletted: банк 6.41% -> лег 6.38% (-0.02%) [7408->7339] флэт
  - nav_ai: банк 14.16% -> лег 13.98% (-0.18%) [16380->16070] флэт
  - items: банк 31.17% -> лег 30.95% (-0.22%) [36051->35581] флэт
  - inside_volatile: банк 12.01% -> лег 11.68% (-0.33%) [13885->13428] флэт
  - fluid: банк 16.72% -> лег 16.10% (-0.62%) [19332->18507] флэт

## VERDICT: **MEASURED**
