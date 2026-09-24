# absorb ROUND (round-443g-mega-1, run 36038730319, branch round-443g-mega-1, head ffb94e4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6728094 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6728094 (поллов=5); TPS_exp=2.22; normalized=+12.8%
- GC: young=106, Full=8, total=19.3s, avg=169ms, max=2526ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105250 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.20% (-0.52%) флэт
  - broadphase: 15.66% -> 9.92% (-5.73%) спад
  - nav_ai: 14.16% -> 3.30% (-10.86%) спад
  - inside_volatile: 12.01% -> 16.48% (+4.48%) РОСТ
  - fastutil: 8.54% -> 6.42% (-2.12%) спад
  - java_util: 7.01% -> 8.58% (+1.57%) РОСТ
  - paletted: 6.41% -> 5.25% (-1.15%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
