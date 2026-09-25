# absorb ROUND (chkmono456-2, run 36122391639, branch round-456c-chunkmono-2, head 270fac2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=24158, col=PARALLEL, runner=7294482 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **FAIL**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.30 @ 7294482 (поллов=5); TPS_exp=2.34; normalized=+41.3%
- GC: young=119, Full=8, total=16.0s, avg=126ms, max=1380ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=101581 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.38% (+0.67%) флэт
  - broadphase: 15.66% -> 10.21% (-5.45%) спад
  - nav_ai: 14.16% -> 2.62% (-11.54%) спад
  - inside_volatile: 12.01% -> 16.52% (+4.52%) РОСТ
  - fastutil: 8.54% -> 6.97% (-1.56%) спад
  - java_util: 7.01% -> 8.27% (+1.26%) РОСТ
  - paletted: 6.41% -> 6.03% (-0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **DELIVERY-FAIL**
