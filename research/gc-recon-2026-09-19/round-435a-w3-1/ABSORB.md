# absorb ROUND (435a-w3-1, run 35924861238, branch round-435a-w3-1, head 062baea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6804217 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.05 @ 6804217 (поллов=6); TPS_exp=2.23; normalized=-8.1%
- GC: young=507, Full=9, total=21.9s, avg=42ms, max=2664ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110859 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.35% (-2.36%) спад
  - broadphase: 15.66% -> 8.69% (-6.97%) спад
  - nav_ai: 14.16% -> 5.95% (-8.21%) спад
  - inside_volatile: 12.01% -> 14.21% (+2.21%) РОСТ
  - fastutil: 8.54% -> 7.28% (-1.25%) спад
  - java_util: 7.01% -> 7.64% (+0.63%) флэт
  - paletted: 6.41% -> 5.76% (-0.65%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
