# absorb ROUND (round-441-chunk4-4, run 35954902635, branch round-441-chunk4-4, head c5fe025)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6867358 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6867358 (поллов=6); TPS_exp=2.25; normalized=+9.1%
- GC: young=108, Full=9, total=19.4s, avg=166ms, max=2652ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104835 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.40% (-0.32%) флэт
  - broadphase: 15.66% -> 9.81% (-5.85%) спад
  - nav_ai: 14.16% -> 3.30% (-10.86%) спад
  - inside_volatile: 12.01% -> 16.23% (+4.22%) РОСТ
  - fastutil: 8.54% -> 6.36% (-2.18%) спад
  - java_util: 7.01% -> 8.07% (+1.06%) РОСТ
  - paletted: 6.41% -> 5.30% (-1.10%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
