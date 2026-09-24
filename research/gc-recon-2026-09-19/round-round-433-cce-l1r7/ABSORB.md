# absorb ROUND (round-433-cce-l1r7, run 35910301051, branch round-433-cce-l1r7, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6972067 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6972067 (поллов=6); TPS_exp=2.27; normalized=+3.7%
- GC: young=101, Full=9, total=20.4s, avg=186ms, max=2624ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103318 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.00% (-0.72%) флэт
  - broadphase: 15.66% -> 9.76% (-5.89%) спад
  - nav_ai: 14.16% -> 3.32% (-10.84%) спад
  - inside_volatile: 12.01% -> 15.47% (+3.46%) РОСТ
  - fastutil: 8.54% -> 6.41% (-2.12%) спад
  - java_util: 7.01% -> 8.63% (+1.62%) РОСТ
  - paletted: 6.41% -> 5.33% (-1.07%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
