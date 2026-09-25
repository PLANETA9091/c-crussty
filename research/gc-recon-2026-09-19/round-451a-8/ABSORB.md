# absorb ROUND (451a-8, run 36069499770, branch round-451-anchor-8, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6735410 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6735410 (поллов=5); TPS_exp=2.22; normalized=+3.7%
- GC: young=114, Full=9, total=23.4s, avg=190ms, max=2621ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115643 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.26% (-1.92%) спад
  - fluid: 16.72% -> 16.59% (-0.12%) флэт
  - broadphase: 15.66% -> 15.14% (-0.52%) флэт
  - nav_ai: 14.16% -> 13.60% (-0.57%) флэт
  - inside_volatile: 12.01% -> 10.90% (-1.11%) спад
  - fastutil: 8.54% -> 8.42% (-0.11%) флэт
  - java_util: 7.01% -> 6.56% (-0.46%) флэт
  - paletted: 6.41% -> 6.76% (+0.35%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
