# absorb MEGA-ROUND-1 items_mono2 (run 35539017250, head 36988be, branch round-396-f-items_mono) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=7101557 (band OK), lever=NOT-ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.30 @ 7101557; TPS_exp(банк-интерполяция)=2.29; DELTA=+0.2% (БЫЛО 2.29 -> СТАЛО 2.30 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=116, Full=9, total=21.3s, avg=170ms, max=2440ms
- T5 ЛЕЙН-КАРТА: total=115230 сэмплов (базлайн 115655)
  - fastutil: банк 8.54% -> лег 9.60% (+1.07%) [9874->11066] РОСТ
  - nav_ai: банк 14.16% -> лег 14.66% (+0.50%) [16380->16895] флэт
  - broadphase: банк 15.66% -> лег 15.88% (+0.23%) [18108->18301] флэт
  - fluid: банк 16.72% -> лег 16.77% (+0.05%) [19332->19322] флэт
  - players_packets: банк 0.01% -> лег 0.00% (-0.01%) [12->5] флэт
  - inside_volatile: банк 12.01% -> лег 11.99% (-0.01%) [13885->13819] флэт
  - paletted: банк 6.41% -> лег 6.35% (-0.06%) [7408->7315] флэт
  - java_util: банк 7.01% -> лег 6.48% (-0.53%) [8110->7468] флэт
  - items: банк 31.17% -> лег 30.64% (-0.53%) [36051->35303] флэт

## VERDICT: **LEVER-NOT-ARMED**
