# absorb ROUND (round-442-chk3-2, run 35957219324, branch round-442-chk3-2, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6860308 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 6860308 (поллов=6); TPS_exp=2.24; normalized=+18.1%
- GC: young=109, Full=9, total=20.1s, avg=171ms, max=2531ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105171 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.81% (+0.09%) флэт
  - broadphase: 15.66% -> 10.17% (-5.48%) спад
  - nav_ai: 14.16% -> 3.19% (-10.98%) спад
  - inside_volatile: 12.01% -> 16.40% (+4.39%) РОСТ
  - fastutil: 8.54% -> 6.66% (-1.88%) спад
  - java_util: 7.01% -> 8.76% (+1.75%) РОСТ
  - paletted: 6.41% -> 5.42% (-0.98%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
