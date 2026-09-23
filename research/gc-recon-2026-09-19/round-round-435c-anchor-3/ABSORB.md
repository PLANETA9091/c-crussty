# absorb ROUND (round-435c-anchor-3, run 35925325052, branch round-435c-anchor-3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6934453 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6934453 (поллов=6); TPS_exp=2.26; normalized=+8.4%
- GC: young=113, Full=10, total=23.3s, avg=189ms, max=2366ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117322 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.66% (-1.51%) спад
  - fluid: 16.72% -> 15.81% (-0.90%) флэт
  - broadphase: 15.66% -> 14.61% (-1.05%) спад
  - nav_ai: 14.16% -> 13.93% (-0.23%) флэт
  - inside_volatile: 12.01% -> 11.63% (-0.37%) флэт
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.39% (-0.62%) флэт
  - paletted: 6.41% -> 6.11% (-0.29%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
