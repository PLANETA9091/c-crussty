# absorb ROUND (round-437-chk3-2, run 35935369774, branch round-437-chk3-2, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7023776 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7023776 (поллов=5); TPS_exp=2.28; normalized=+9.7%
- GC: young=106, Full=9, total=20.5s, avg=178ms, max=2924ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105058 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.10% (-0.61%) флэт
  - broadphase: 15.66% -> 10.18% (-5.47%) спад
  - nav_ai: 14.16% -> 3.39% (-10.78%) спад
  - inside_volatile: 12.01% -> 16.14% (+4.14%) РОСТ
  - fastutil: 8.54% -> 6.44% (-2.10%) спад
  - java_util: 7.01% -> 8.42% (+1.41%) РОСТ
  - paletted: 6.41% -> 5.41% (-0.99%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
