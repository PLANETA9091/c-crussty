# absorb ROUND (433-anchor-a, run 35901431136, branch round-433-anchor-a, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6711021 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 6711021 (поллов=5); TPS_exp=2.21; normalized=-14.1%
- GC: young=103, Full=10, total=24.2s, avg=214ms, max=3014ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115591 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.24% (-1.93%) спад
  - fluid: 16.72% -> 15.62% (-1.09%) спад
  - broadphase: 15.66% -> 15.40% (-0.26%) флэт
  - nav_ai: 14.16% -> 13.75% (-0.42%) флэт
  - inside_volatile: 12.01% -> 11.06% (-0.95%) флэт
  - fastutil: 8.54% -> 8.78% (+0.24%) флэт
  - java_util: 7.01% -> 7.15% (+0.14%) флэт
  - paletted: 6.41% -> 6.10% (-0.31%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
