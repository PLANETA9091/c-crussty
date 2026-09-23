# absorb ROUND (c424l1, run 35827054415, branch round-424-c-l1, head 6f92ea7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9399955 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.40 @ 9399955 (поллов=5); TPS_exp=2.78; normalized=+22.4%
- GC: young=1173, Full=10, total=31.4s, avg=27ms, max=2585ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107874 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.85% (+0.13%) флэт
  - broadphase: 15.66% -> 13.35% (-2.30%) спад
  - nav_ai: 14.16% -> 9.45% (-4.71%) спад
  - inside_volatile: 12.01% -> 13.05% (+1.05%) РОСТ
  - fastutil: 8.54% -> 7.72% (-0.82%) флэт
  - java_util: 7.01% -> 8.26% (+1.24%) РОСТ
  - paletted: 6.41% -> 6.64% (+0.24%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
