# absorb ROUND (round412multi4, run 35709605852, branch round-412-multi4, head f7d04d2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6952974 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 6952974 (поллов=5); TPS_exp=2.26; normalized=+28.1%
- GC: young=109, Full=9, total=18.2s, avg=154ms, max=2150ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110564 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.09% (-0.63%) флэт
  - broadphase: 15.66% -> 14.23% (-1.43%) спад
  - nav_ai: 14.16% -> 9.43% (-4.73%) спад
  - inside_volatile: 12.01% -> 12.79% (+0.78%) флэт
  - fastutil: 8.54% -> 7.96% (-0.57%) флэт
  - java_util: 7.01% -> 7.81% (+0.80%) флэт
  - paletted: 6.41% -> 5.60% (-0.81%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
