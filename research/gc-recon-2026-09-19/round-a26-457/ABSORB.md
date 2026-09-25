# absorb ROUND (a26-457, run 36135816829, branch round-457-anchor-26, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8671791 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.95 @ 8671791 (поллов=6); TPS_exp=2.63; normalized=+12.4%
- GC: young=117, Full=10, total=20.1s, avg=159ms, max=2092ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112962 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.71% (-2.46%) спад
  - fluid: 16.72% -> 16.18% (-0.54%) флэт
  - broadphase: 15.66% -> 14.93% (-0.72%) флэт
  - nav_ai: 14.16% -> 13.73% (-0.43%) флэт
  - inside_volatile: 12.01% -> 10.44% (-1.57%) спад
  - fastutil: 8.54% -> 8.78% (+0.24%) флэт
  - java_util: 7.01% -> 6.83% (-0.19%) флэт
  - paletted: 6.41% -> 6.78% (+0.37%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
