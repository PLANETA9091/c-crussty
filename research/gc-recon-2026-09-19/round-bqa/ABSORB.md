# absorb ROUND (bqa, run 35764266923, branch round-417-c-bqa, head bad02d8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8960321 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8960321 (поллов=5); TPS_exp=2.69; normalized=+19.1%
- GC: young=1201, Full=9, total=24.4s, avg=20ms, max=1788ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105902 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.69% (-0.03%) флэт
  - broadphase: 15.66% -> 13.67% (-1.98%) спад
  - nav_ai: 14.16% -> 9.25% (-4.92%) спад
  - inside_volatile: 12.01% -> 11.23% (-0.77%) флэт
  - fastutil: 8.54% -> 8.36% (-0.18%) флэт
  - java_util: 7.01% -> 8.04% (+1.03%) РОСТ
  - paletted: 6.41% -> 5.99% (-0.42%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
