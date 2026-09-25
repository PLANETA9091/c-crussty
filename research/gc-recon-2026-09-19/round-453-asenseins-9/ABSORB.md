# absorb ROUND (453-asenseins-9, run 36090378630, branch round-453-asenseins-9, head 76d9798)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8924510 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.00 @ 8924510 (поллов=5); TPS_exp=2.68; normalized=+12.0%
- GC: young=108, Full=9, total=22.1s, avg=189ms, max=3058ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105004 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.11% (+1.40%) РОСТ
  - broadphase: 15.66% -> 10.55% (-5.11%) спад
  - nav_ai: 14.16% -> 3.24% (-10.93%) спад
  - inside_volatile: 12.01% -> 17.74% (+5.73%) РОСТ
  - fastutil: 8.54% -> 6.62% (-1.92%) спад
  - java_util: 7.01% -> 8.59% (+1.58%) РОСТ
  - paletted: 6.41% -> 6.55% (+0.14%) флэт
  - players_packets: 0.01% -> 0.02% (+0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
