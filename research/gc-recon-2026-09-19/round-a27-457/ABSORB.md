# absorb ROUND (a27-457, run 36135828969, branch round-457-anchor-27, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7058810 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.70 @ 7058810 (поллов=6); TPS_exp=2.29; normalized=-25.6%
- GC: young=105, Full=9, total=24.6s, avg=216ms, max=2657ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115820 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.54% (-1.63%) спад
  - fluid: 16.72% -> 15.91% (-0.80%) флэт
  - broadphase: 15.66% -> 15.27% (-0.39%) флэт
  - nav_ai: 14.16% -> 13.15% (-1.01%) спад
  - inside_volatile: 12.01% -> 10.92% (-1.08%) спад
  - fastutil: 8.54% -> 8.46% (-0.08%) флэт
  - java_util: 7.01% -> 6.46% (-0.55%) флэт
  - paletted: 6.41% -> 6.22% (-0.19%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
