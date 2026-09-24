# absorb ROUND (450b-anchor-18, run 36055353775, branch round-450-anchor-18, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6463213 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6463213 (поллов=6); TPS_exp=2.16; normalized=+4.2%
- GC: young=110, Full=9, total=23.0s, avg=194ms, max=2610ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115268 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.37% (-1.80%) спад
  - fluid: 16.72% -> 16.56% (-0.16%) флэт
  - broadphase: 15.66% -> 15.20% (-0.46%) флэт
  - nav_ai: 14.16% -> 13.60% (-0.56%) флэт
  - inside_volatile: 12.01% -> 10.45% (-1.55%) спад
  - fastutil: 8.54% -> 8.66% (+0.12%) флэт
  - java_util: 7.01% -> 6.21% (-0.80%) флэт
  - paletted: 6.41% -> 6.98% (+0.57%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
