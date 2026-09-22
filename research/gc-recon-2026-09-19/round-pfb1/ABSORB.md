# absorb ROUND (pfb1, run 35733825772, branch round-414-c-pfb1, head e76cb51)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6669214 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6669214 (поллов=6); TPS_exp=2.20; normalized=+18.0%
- GC: young=4142, Full=11, total=44.7s, avg=11ms, max=2376ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113234 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.91% (-1.80%) спад
  - broadphase: 15.66% -> 16.48% (+0.82%) флэт
  - nav_ai: 14.16% -> 15.66% (+1.50%) РОСТ
  - inside_volatile: 12.01% -> 12.14% (+0.13%) флэт
  - fastutil: 8.54% -> 9.45% (+0.91%) флэт
  - java_util: 7.01% -> 7.85% (+0.84%) флэт
  - paletted: 6.41% -> 5.55% (-0.85%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
