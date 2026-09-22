# absorb ROUND (mc4a, run 35788907864, branch round-420-mc4a, head 20c9fdc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6496785 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6496785 (поллов=5); TPS_exp=2.17; normalized=+1.5%
- GC: young=106, Full=9, total=19.0s, avg=165ms, max=2397ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107172 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.86% (+0.15%) флэт
  - broadphase: 15.66% -> 9.71% (-5.95%) спад
  - nav_ai: 14.16% -> 4.25% (-9.91%) спад
  - inside_volatile: 12.01% -> 11.47% (-0.54%) флэт
  - fastutil: 8.54% -> 5.88% (-2.66%) спад
  - java_util: 7.01% -> 7.58% (+0.57%) флэт
  - paletted: 6.41% -> 6.14% (-0.27%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
