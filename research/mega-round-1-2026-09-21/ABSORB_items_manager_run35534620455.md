# absorb MEGA-ROUND-1 items_manager (run 35534620455, head 4e8fc34, branch round-395-j-manager) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=6775348 (band OK), lever=ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **PASS**
  - arm-marker: [20:16:02 INFO]: [crussty-plugin] [crussty-plugin] items_manager: bridge ready (enabled=true)
  - arm-marker: [20:16:02 INFO]: [crussty-plugin] [crussty-plugin] items_manager: bridge ready (enabled=true)
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.30 @ 6775348; TPS_exp(банк-интерполяция)=2.23; DELTA=+3.3% (БЫЛО 2.23 -> СТАЛО 2.30 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=116, Full=9, total=21.9s, avg=175ms, max=2480ms
- T5 ЛЕЙН-КАРТА: total=113737 сэмплов (базлайн 115655)
  - nav_ai: банк 14.16% -> лег 14.56% (+0.39%) [16380->16557] флэт
  - fastutil: банк 8.54% -> лег 8.88% (+0.34%) [9874->10099] флэт
  - inside_volatile: банк 12.01% -> лег 12.22% (+0.22%) [13885->13903] флэт
  - paletted: банк 6.41% -> лег 6.61% (+0.21%) [7408->7523] флэт
  - broadphase: банк 15.66% -> лег 15.69% (+0.04%) [18108->17849] флэт
  - players_packets: банк 0.01% -> лег 0.01% (-0.00%) [12->7] флэт
  - java_util: банк 7.01% -> лег 6.57% (-0.44%) [8110->7478] флэт
  - fluid: банк 16.72% -> лег 16.08% (-0.64%) [19332->18287] флэт
  - items: банк 31.17% -> лег 0.00% (-31.17%) [36051->0] спад

## VERDICT: **MEASURED**
