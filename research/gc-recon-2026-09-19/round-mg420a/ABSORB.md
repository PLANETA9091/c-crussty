# absorb ROUND (mg420a, run 35795518280, branch round-420-mga, head 77b22b7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6777027 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 6777027 (поллов=6); TPS_exp=2.23; normalized=+23.5%
- GC: young=1136, Full=9, total=25.2s, avg=22ms, max=2280ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108362 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.04% (-0.67%) флэт
  - broadphase: 15.66% -> 13.88% (-1.78%) спад
  - nav_ai: 14.16% -> 8.97% (-5.19%) спад
  - inside_volatile: 12.01% -> 12.58% (+0.58%) флэт
  - fastutil: 8.54% -> 7.66% (-0.88%) флэт
  - java_util: 7.01% -> 7.86% (+0.85%) флэт
  - paletted: 6.41% -> 5.48% (-0.93%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
