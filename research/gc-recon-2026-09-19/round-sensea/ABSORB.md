# absorb ROUND (sensea, run 35779689421, branch round-419-b-sba, head a6cf691)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7016129 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7016129 (поллов=5); TPS_exp=2.28; normalized=+9.8%
- GC: young=1135, Full=9, total=25.0s, avg=22ms, max=2073ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108878 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.19% (-0.53%) флэт
  - broadphase: 15.66% -> 13.78% (-1.87%) спад
  - nav_ai: 14.16% -> 9.32% (-4.84%) спад
  - inside_volatile: 12.01% -> 11.90% (-0.10%) флэт
  - fastutil: 8.54% -> 7.81% (-0.73%) флэт
  - java_util: 7.01% -> 7.76% (+0.75%) флэт
  - paletted: 6.41% -> 5.43% (-0.98%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
