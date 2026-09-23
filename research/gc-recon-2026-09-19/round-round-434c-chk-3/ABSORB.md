# absorb ROUND (round-434c-chk-3, run 35917673383, branch round-434c-chk-3, head a17cde0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8675334 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8675334 (поллов=5); TPS_exp=2.63; normalized=+21.9%
- GC: young=130, Full=7, total=15.7s, avg=115ms, max=1386ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=101657 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.51% (-0.21%) флэт
  - broadphase: 15.66% -> 9.22% (-6.44%) спад
  - nav_ai: 14.16% -> 3.34% (-10.83%) спад
  - inside_volatile: 12.01% -> 15.30% (+3.29%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.21%) спад
  - java_util: 7.01% -> 8.40% (+1.39%) РОСТ
  - paletted: 6.41% -> 5.98% (-0.42%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
