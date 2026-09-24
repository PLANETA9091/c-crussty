# absorb ROUND (451c-chunk-7, run 36070495370, branch round-451c-chunk-7, head 29ac648)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8923320 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8923320 (поллов=5); TPS_exp=2.68; normalized=+19.5%
- GC: young=124, Full=10, total=21.6s, avg=161ms, max=2171ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102541 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.44% (-0.28%) флэт
  - broadphase: 15.66% -> 9.26% (-6.40%) спад
  - nav_ai: 14.16% -> 3.15% (-11.02%) спад
  - inside_volatile: 12.01% -> 15.05% (+3.04%) РОСТ
  - fastutil: 8.54% -> 6.11% (-2.42%) спад
  - java_util: 7.01% -> 8.42% (+1.41%) РОСТ
  - paletted: 6.41% -> 5.97% (-0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
