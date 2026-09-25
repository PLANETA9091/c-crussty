# absorb ROUND (453-asenseins-7, run 36090362055, branch round-453-asenseins-7, head 76d9798)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6502763 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 6502763 (поллов=6); TPS_exp=2.17; normalized=+33.7%
- GC: young=109, Full=9, total=21.4s, avg=181ms, max=2649ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103938 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.90% (+1.19%) РОСТ
  - broadphase: 15.66% -> 10.74% (-4.92%) спад
  - nav_ai: 14.16% -> 3.09% (-11.08%) спад
  - inside_volatile: 12.01% -> 17.02% (+5.02%) РОСТ
  - fastutil: 8.54% -> 7.09% (-1.45%) спад
  - java_util: 7.01% -> 8.92% (+1.91%) РОСТ
  - paletted: 6.41% -> 6.40% (-0.01%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
