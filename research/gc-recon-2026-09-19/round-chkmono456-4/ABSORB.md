# absorb ROUND (chkmono456-4, run 36125434175, branch round-456c-chunkmono-4, head 097def9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8650222 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8650222 (поллов=5); TPS_exp=2.62; normalized=-0.8%
- GC: young=107, Full=9, total=23.0s, avg=198ms, max=3123ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105960 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.89% (+1.18%) РОСТ
  - broadphase: 15.66% -> 9.83% (-5.83%) спад
  - nav_ai: 14.16% -> 3.22% (-10.94%) спад
  - inside_volatile: 12.01% -> 17.18% (+5.17%) РОСТ
  - fastutil: 8.54% -> 6.56% (-1.98%) спад
  - java_util: 7.01% -> 8.88% (+1.86%) РОСТ
  - paletted: 6.41% -> 6.35% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
