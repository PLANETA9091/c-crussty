# absorb ROUND (round-443g-anchor-11, run 36038743347, branch round-443g-anchor-11, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6822062 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6822062 (поллов=6); TPS_exp=2.24; normalized=+0.6%
- GC: young=111, Full=10, total=24.5s, avg=203ms, max=2480ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117057 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.92% (-1.25%) спад
  - fluid: 16.72% -> 15.95% (-0.76%) флэт
  - broadphase: 15.66% -> 15.68% (+0.03%) флэт
  - nav_ai: 14.16% -> 13.99% (-0.17%) флэт
  - inside_volatile: 12.01% -> 11.32% (-0.68%) флэт
  - fastutil: 8.54% -> 8.78% (+0.25%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 6.23% (-0.18%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
