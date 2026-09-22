# absorb ROUND (senseb, run 35779697793, branch round-419-b-sbb, head a6cf691)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6605711 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6605711 (поллов=5); TPS_exp=2.19; normalized=+14.2%
- GC: young=1130, Full=9, total=25.8s, avg=23ms, max=2211ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107879 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.61% (-1.11%) спад
  - broadphase: 15.66% -> 13.75% (-1.91%) спад
  - nav_ai: 14.16% -> 8.89% (-5.27%) спад
  - inside_volatile: 12.01% -> 12.22% (+0.21%) флэт
  - fastutil: 8.54% -> 7.66% (-0.88%) флэт
  - java_util: 7.01% -> 7.74% (+0.72%) флэт
  - paletted: 6.41% -> 5.60% (-0.80%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
