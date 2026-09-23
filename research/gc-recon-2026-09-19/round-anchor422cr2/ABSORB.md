# absorb ROUND (anchor422cr2, run 35811903429, branch round-422-anchorcr2, head 57a2e67)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6818153 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6818153 (поллов=6); TPS_exp=2.23; normalized=-6.0%
- GC: young=106, Full=9, total=21.3s, avg=185ms, max=2545ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115551 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.16% (-2.01%) спад
  - fluid: 16.72% -> 15.45% (-1.27%) спад
  - broadphase: 15.66% -> 15.07% (-0.59%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.49%) флэт
  - inside_volatile: 12.01% -> 10.88% (-1.12%) спад
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.43% (-0.58%) флэт
  - paletted: 6.41% -> 5.81% (-0.59%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
