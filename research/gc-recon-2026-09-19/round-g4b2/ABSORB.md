# absorb ROUND (g4b2, run 35754653864, branch round-416-b-g4b, head 89b3bff)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7043361 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7043361 (поллов=6); TPS_exp=2.28; normalized=+5.2%
- GC: young=106, Full=10, total=23.2s, avg=200ms, max=2617ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112417 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.81% (-1.91%) спад
  - broadphase: 15.66% -> 15.51% (-0.15%) флэт
  - nav_ai: 14.16% -> 7.02% (-7.14%) спад
  - inside_volatile: 12.01% -> 11.68% (-0.33%) флэт
  - fastutil: 8.54% -> 8.58% (+0.04%) флэт
  - java_util: 7.01% -> 7.49% (+0.48%) флэт
  - paletted: 6.41% -> 5.52% (-0.89%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
