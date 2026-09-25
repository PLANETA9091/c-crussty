# absorb ROUND (a16-457, run 36131742213, branch round-457-anchor-16, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6843586 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6843586 (поллов=5); TPS_exp=2.24; normalized=-1.8%
- GC: young=109, Full=9, total=22.9s, avg=194ms, max=2589ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116164 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.88% (-1.29%) спад
  - fluid: 16.72% -> 17.10% (+0.38%) флэт
  - broadphase: 15.66% -> 14.64% (-1.02%) спад
  - nav_ai: 14.16% -> 13.59% (-0.58%) флэт
  - inside_volatile: 12.01% -> 10.74% (-1.26%) спад
  - fastutil: 8.54% -> 8.83% (+0.29%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 7.02% (+0.62%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
