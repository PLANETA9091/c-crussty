# absorb ROUND (round-435-b-ins-r2, run 35924407887, branch round-435-b-ins-r2, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6643675 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6643675 (поллов=6); TPS_exp=2.20; normalized=+13.7%
- GC: young=108, Full=5, total=14.9s, avg=132ms, max=547ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104172 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.61% (+0.90%) флэт
  - broadphase: 15.66% -> 9.53% (-6.13%) спад
  - nav_ai: 14.16% -> 3.14% (-11.02%) спад
  - inside_volatile: 12.01% -> 16.91% (+4.91%) РОСТ
  - fastutil: 8.54% -> 6.26% (-2.27%) спад
  - java_util: 7.01% -> 8.33% (+1.32%) РОСТ
  - paletted: 6.41% -> 5.56% (-0.84%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
