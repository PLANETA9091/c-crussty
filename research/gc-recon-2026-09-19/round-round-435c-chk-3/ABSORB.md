# absorb ROUND (round-435c-chk-3, run 35925935995, branch round-435c-chk-3, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9168315 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.00 @ 9168315 (поллов=5); TPS_exp=2.73; normalized=+9.9%
- GC: young=116, Full=9, total=21.3s, avg=171ms, max=2768ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106382 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.19% (+0.47%) флэт
  - broadphase: 15.66% -> 9.66% (-6.00%) спад
  - nav_ai: 14.16% -> 3.54% (-10.62%) спад
  - inside_volatile: 12.01% -> 16.25% (+4.25%) РОСТ
  - fastutil: 8.54% -> 6.59% (-1.95%) спад
  - java_util: 7.01% -> 9.01% (+1.99%) РОСТ
  - paletted: 6.41% -> 6.32% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
