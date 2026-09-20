# absorb MEGA-ROUND-1 items_stagger (run 35542071870, head 5867c17, branch round-396-b-items_stagger) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=8513544 (band OK), lever=NOT-ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.20 @ 8513544; TPS_exp(банк-интерполяция)=2.59; DELTA=-15.1% (БЫЛО 2.59 -> СТАЛО 2.20 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=110, Full=9, total=24.9s, avg=210ms, max=3326ms
- T5 ЛЕЙН-КАРТА: total=115473 сэмплов (базлайн 115655)
  - fluid: банк 16.72% -> лег 18.44% (+1.72%) [19332->21291] РОСТ
  - paletted: банк 6.41% -> лег 7.66% (+1.26%) [7408->8846] РОСТ
  - fastutil: банк 8.54% -> лег 9.30% (+0.76%) [9874->10735] флэт
  - nav_ai: банк 14.16% -> лег 14.67% (+0.51%) [16380->16943] флэт
  - broadphase: банк 15.66% -> лег 15.94% (+0.29%) [18108->18410] флэт
  - java_util: банк 7.01% -> лег 7.06% (+0.05%) [8110->8151] флэт
  - players_packets: банк 0.01% -> лег 0.01% (-0.00%) [12->10] флэт
  - inside_volatile: банк 12.01% -> лег 11.58% (-0.43%) [13885->13372] флэт
  - items: банк 31.17% -> лег 30.53% (-0.64%) [36051->35256] флэт

## VERDICT: **LEVER-NOT-ARMED**
