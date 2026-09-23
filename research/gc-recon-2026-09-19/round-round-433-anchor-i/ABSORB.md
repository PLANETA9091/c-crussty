# absorb ROUND (round-433-anchor-i, run 35907631148, branch round-433-anchor-i, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8997116 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8997116 (поллов=5); TPS_exp=2.69; normalized=-3.5%
- GC: young=116, Full=10, total=21.9s, avg=174ms, max=2141ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113248 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.55% (-2.62%) спад
  - fluid: 16.72% -> 16.24% (-0.47%) флэт
  - broadphase: 15.66% -> 14.92% (-0.73%) флэт
  - nav_ai: 14.16% -> 14.16% (-0.00%) флэт
  - inside_volatile: 12.01% -> 10.34% (-1.67%) спад
  - fastutil: 8.54% -> 9.29% (+0.76%) флэт
  - java_util: 7.01% -> 6.42% (-0.59%) флэт
  - paletted: 6.41% -> 6.90% (+0.49%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
