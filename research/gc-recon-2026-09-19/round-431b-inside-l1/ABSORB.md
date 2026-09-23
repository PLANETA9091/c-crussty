# absorb ROUND (431b-inside-l1, run 35883917278, branch round-431b-inside-l1, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7177515 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 7177515 (поллов=5); TPS_exp=2.31; normalized=+34.2%
- GC: young=115, Full=9, total=23.4s, avg=188ms, max=3389ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104518 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.49% (+1.77%) РОСТ
  - broadphase: 15.66% -> 10.21% (-5.44%) спад
  - nav_ai: 14.16% -> 3.78% (-10.39%) спад
  - inside_volatile: 12.01% -> 15.91% (+3.90%) РОСТ
  - fastutil: 8.54% -> 6.07% (-2.47%) спад
  - java_util: 7.01% -> 8.19% (+1.18%) РОСТ
  - paletted: 6.41% -> 5.86% (-0.54%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
