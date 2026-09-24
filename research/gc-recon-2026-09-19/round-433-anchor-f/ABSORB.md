# absorb ROUND (433-anchor-f, run 35901570487, branch round-433-anchor-f, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6119324 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6119324 (поллов=5); TPS_exp=2.09; normalized=+10.2%
- GC: young=116, Full=10, total=23.7s, avg=188ms, max=2413ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117287 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.06% (-1.11%) спад
  - fluid: 16.72% -> 15.96% (-0.76%) флэт
  - broadphase: 15.66% -> 14.62% (-1.03%) спад
  - nav_ai: 14.16% -> 14.01% (-0.15%) флэт
  - inside_volatile: 12.01% -> 12.26% (+0.26%) флэт
  - fastutil: 8.54% -> 8.83% (+0.29%) флэт
  - java_util: 7.01% -> 6.87% (-0.14%) флэт
  - paletted: 6.41% -> 6.10% (-0.30%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
