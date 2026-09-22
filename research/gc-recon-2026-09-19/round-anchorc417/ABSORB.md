# absorb ROUND (anchorc417, run 35759273462, branch round-417-anchorc, head 87fc0cb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6761869 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6761869 (поллов=6); TPS_exp=2.22; normalized=+8.0%
- GC: young=111, Full=9, total=21.1s, avg=176ms, max=2448ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115042 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.37% (-0.80%) флэт
  - fluid: 16.72% -> 16.44% (-0.28%) флэт
  - broadphase: 15.66% -> 15.49% (-0.16%) флэт
  - nav_ai: 14.16% -> 14.57% (+0.40%) флэт
  - inside_volatile: 12.01% -> 12.03% (+0.02%) флэт
  - fastutil: 8.54% -> 9.29% (+0.75%) флэт
  - java_util: 7.01% -> 8.48% (+1.47%) РОСТ
  - paletted: 6.41% -> 6.70% (+0.29%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
