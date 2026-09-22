# absorb ROUND (mc2b, run 35748267454, branch round-416-a-mc2b, head 079eb4a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6972794 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=0.20 @ 6972794 (поллов=5); TPS_exp=2.27; normalized=-91.2%
- GC: young=60, Full=8, total=10.3s, avg=151ms, max=1572ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111823 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 10.95% (-5.76%) спад
  - broadphase: 15.66% -> 7.18% (-8.48%) спад
  - nav_ai: 14.16% -> 2.88% (-11.28%) спад
  - inside_volatile: 12.01% -> 8.83% (-3.18%) спад
  - fastutil: 8.54% -> 4.14% (-4.40%) спад
  - java_util: 7.01% -> 7.07% (+0.05%) флэт
  - paletted: 6.41% -> 3.90% (-2.51%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
