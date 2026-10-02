# absorb MEGA-ROUND-1 items_oss (run 35541378756, head 0f3af53, branch round-396-h-items_oss) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=6868400 (band OK), lever=NOT-ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.40 @ 6868400; TPS_exp(банк-интерполяция)=2.25; DELTA=+6.9% (БЫЛО 2.25 -> СТАЛО 2.40 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=108, Full=9, total=20.0s, avg=171ms, max=2451ms
- T5 ЛЕЙН-КАРТА: total=114472 сэмплов (базлайн 115655)
  - nav_ai: банк 14.16% -> лег 14.88% (+0.72%) [16380->17032] флэт
  - fastutil: банк 8.54% -> лег 8.95% (+0.41%) [9874->10241] флэт
  - broadphase: банк 15.66% -> лег 15.89% (+0.23%) [18108->18187] флэт
  - paletted: банк 6.41% -> лег 6.50% (+0.09%) [7408->7436] флэт
  - players_packets: банк 0.01% -> лег 0.01% (+0.00%) [12->13] флэт
  - fluid: банк 16.72% -> лег 16.63% (-0.09%) [19332->19031] флэт
  - java_util: банк 7.01% -> лег 6.91% (-0.10%) [8110->7908] флэт
  - inside_volatile: банк 12.01% -> лег 11.60% (-0.41%) [13885->13276] флэт
  - items: банк 31.17% -> лег 30.64% (-0.53%) [36051->35074] флэт

## VERDICT: **LEVER-NOT-ARMED**
