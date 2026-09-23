# absorb ROUND (round-435-b-ins-r4, run 35924503657, branch round-435-b-ins-r4, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7059731 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7059731 (поллов=5); TPS_exp=2.29; normalized=+0.6%
- GC: young=105, Full=9, total=20.3s, avg=178ms, max=2948ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104363 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.49% (-0.22%) флэт
  - broadphase: 15.66% -> 9.87% (-5.79%) спад
  - nav_ai: 14.16% -> 3.28% (-10.88%) спад
  - inside_volatile: 12.01% -> 16.40% (+4.39%) РОСТ
  - fastutil: 8.54% -> 6.37% (-2.16%) спад
  - java_util: 7.01% -> 8.27% (+1.26%) РОСТ
  - paletted: 6.41% -> 5.39% (-1.01%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
