# absorb ROUND (round-440c-chunk4-2, run 35951322749, branch round-440c-chunk4-2, head 8938afb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6851108 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6851108 (поллов=5); TPS_exp=2.24; normalized=+11.5%
- GC: young=109, Full=9, total=19.8s, avg=168ms, max=2463ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103530 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.97% (-0.75%) флэт
  - broadphase: 15.66% -> 9.83% (-5.83%) спад
  - nav_ai: 14.16% -> 3.50% (-10.66%) спад
  - inside_volatile: 12.01% -> 15.88% (+3.88%) РОСТ
  - fastutil: 8.54% -> 6.82% (-1.72%) спад
  - java_util: 7.01% -> 8.42% (+1.41%) РОСТ
  - paletted: 6.41% -> 5.21% (-1.20%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
