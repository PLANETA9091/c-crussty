# absorb ROUND (diet455-3, run 36104870859, branch round-455c-diet-3, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7315179 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 7315179 (поллов=5); TPS_exp=2.34; normalized=+19.7%
- GC: young=113, Full=9, total=22.2s, avg=182ms, max=3097ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106001 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.17% (+1.45%) РОСТ
  - broadphase: 15.66% -> 9.97% (-5.69%) спад
  - nav_ai: 14.16% -> 3.41% (-10.75%) спад
  - inside_volatile: 12.01% -> 17.87% (+5.86%) РОСТ
  - fastutil: 8.54% -> 5.88% (-2.66%) спад
  - java_util: 7.01% -> 8.69% (+1.68%) РОСТ
  - paletted: 6.41% -> 5.43% (-0.97%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
