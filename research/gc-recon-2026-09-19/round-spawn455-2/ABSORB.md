# absorb ROUND (spawn455-2, run 36106535213, branch round-455a-spawn-2, head c25782b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8769142 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8769142 (поллов=5); TPS_exp=2.65; normalized=+2.0%
- GC: young=105, Full=8, total=22.0s, avg=194ms, max=2999ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106350 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.66% (+0.94%) флэт
  - broadphase: 15.66% -> 9.41% (-6.24%) спад
  - nav_ai: 14.16% -> 3.61% (-10.55%) спад
  - inside_volatile: 12.01% -> 17.25% (+5.25%) РОСТ
  - fastutil: 8.54% -> 6.57% (-1.97%) спад
  - java_util: 7.01% -> 9.21% (+2.20%) РОСТ
  - paletted: 6.41% -> 6.62% (+0.21%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
