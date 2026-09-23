# absorb ROUND (round-433-cce-l1r4, run 35901444361, branch round-433-cce-l1r4, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8882831 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.30 @ 8882831 (поллов=5); TPS_exp=2.67; normalized=+23.6%
- GC: young=136, Full=10, total=20.1s, avg=138ms, max=2113ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102356 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.34% (-0.37%) флэт
  - broadphase: 15.66% -> 9.02% (-6.64%) спад
  - nav_ai: 14.16% -> 3.21% (-10.95%) спад
  - inside_volatile: 12.01% -> 14.92% (+2.91%) РОСТ
  - fastutil: 8.54% -> 6.27% (-2.26%) спад
  - java_util: 7.01% -> 8.49% (+1.48%) РОСТ
  - paletted: 6.41% -> 5.95% (-0.45%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
