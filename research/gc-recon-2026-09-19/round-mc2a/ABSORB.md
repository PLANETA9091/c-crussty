# absorb ROUND (mc2a, run 35748247988, branch round-416-a-mc2a, head 079eb4a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6833856 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=0.20 @ 6833856 (поллов=5); TPS_exp=2.24; normalized=-91.1%
- GC: young=62, Full=8, total=11.2s, avg=160ms, max=1850ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111086 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 11.72% (-4.99%) спад
  - broadphase: 15.66% -> 7.56% (-8.09%) спад
  - nav_ai: 14.16% -> 2.84% (-11.32%) спад
  - inside_volatile: 12.01% -> 8.80% (-3.21%) спад
  - fastutil: 8.54% -> 4.02% (-4.51%) спад
  - java_util: 7.01% -> 7.81% (+0.80%) флэт
  - paletted: 6.41% -> 3.95% (-2.46%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
