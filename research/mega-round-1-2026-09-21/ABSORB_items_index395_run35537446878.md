# absorb MEGA-ROUND-1 items_index395 (run 35537446878, head 7eb5502, branch round-395-a-merge-index) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=6939260 (band OK), lever=NOT-ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.10 @ 6939260; TPS_exp(банк-интерполяция)=2.26; DELTA=-7.1% (БЫЛО 2.26 -> СТАЛО 2.10 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=110, Full=9, total=21.8s, avg=184ms, max=2833ms
- T5 ЛЕЙН-КАРТА: total=114847 сэмплов (базлайн 115655)
  - fastutil: банк 8.54% -> лег 9.72% (+1.19%) [9874->11166] РОСТ
  - nav_ai: банк 14.16% -> лег 14.74% (+0.58%) [16380->16932] флэт
  - broadphase: банк 15.66% -> лег 15.97% (+0.31%) [18108->18342] флэт
  - items: банк 31.17% -> лег 31.21% (+0.04%) [36051->35841] флэт
  - java_util: банк 7.01% -> лег 7.02% (+0.01%) [8110->8068] флэт
  - players_packets: банк 0.01% -> лег 0.01% (-0.00%) [12->10] флэт
  - paletted: банк 6.41% -> лег 6.40% (-0.01%) [7408->7347] флэт
  - inside_volatile: банк 12.01% -> лег 11.79% (-0.22%) [13885->13541] флэт
  - fluid: банк 16.72% -> лег 16.19% (-0.53%) [19332->18593] флэт

## VERDICT: **LEVER-NOT-ARMED**
