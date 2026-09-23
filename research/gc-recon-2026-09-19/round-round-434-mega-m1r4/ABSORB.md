# absorb ROUND (round-434-mega-m1r4, run 35919394865, branch round-434-mega-m1r4, head b15f329)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6940301 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6940301 (поллов=5); TPS_exp=2.26; normalized=+15.0%
- GC: young=109, Full=9, total=19.4s, avg=165ms, max=2629ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103482 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.98% (+0.26%) флэт
  - broadphase: 15.66% -> 10.35% (-5.31%) спад
  - nav_ai: 14.16% -> 3.56% (-10.60%) спад
  - inside_volatile: 12.01% -> 13.56% (+1.55%) РОСТ
  - fastutil: 8.54% -> 6.77% (-1.77%) спад
  - java_util: 7.01% -> 7.58% (+0.57%) флэт
  - paletted: 6.41% -> 5.82% (-0.58%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
