# absorb ROUND (a23-457, run 36135778634, branch round-457-anchor-23, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6793237 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6793237 (поллов=5); TPS_exp=2.23; normalized=-5.8%
- GC: young=111, Full=10, total=23.6s, avg=195ms, max=2402ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116746 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.26% (-1.91%) спад
  - fluid: 16.72% -> 15.51% (-1.21%) спад
  - broadphase: 15.66% -> 15.43% (-0.23%) флэт
  - nav_ai: 14.16% -> 13.59% (-0.57%) флэт
  - inside_volatile: 12.01% -> 11.38% (-0.63%) флэт
  - fastutil: 8.54% -> 8.75% (+0.22%) флэт
  - java_util: 7.01% -> 6.27% (-0.74%) флэт
  - paletted: 6.41% -> 6.32% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
