# absorb ROUND (a11-456, run 36112183078, branch round-456-anchor-11, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8717650 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8717650 (поллов=6); TPS_exp=2.63; normalized=-5.1%
- GC: young=113, Full=10, total=28.8s, avg=234ms, max=3058ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117032 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.04% (-1.13%) спад
  - fluid: 16.72% -> 18.33% (+1.62%) РОСТ
  - broadphase: 15.66% -> 15.03% (-0.63%) флэт
  - nav_ai: 14.16% -> 13.74% (-0.43%) флэт
  - inside_volatile: 12.01% -> 11.39% (-0.62%) флэт
  - fastutil: 8.54% -> 8.65% (+0.11%) флэт
  - java_util: 7.01% -> 6.59% (-0.42%) флэт
  - paletted: 6.41% -> 7.62% (+1.21%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
