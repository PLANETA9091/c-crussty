# absorb ROUND (round-436b-anchor-b36, run 35932087903, branch round-436b-anchor-b36, head e05994c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6566994 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6566994 (поллов=6); TPS_exp=2.18; normalized=+7.7%
- GC: young=110, Full=9, total=23.0s, avg=193ms, max=2650ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115273 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.64% (-1.53%) спад
  - fluid: 16.72% -> 17.00% (+0.29%) флэт
  - broadphase: 15.66% -> 14.71% (-0.94%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.67%) флэт
  - inside_volatile: 12.01% -> 10.61% (-1.39%) спад
  - fastutil: 8.54% -> 8.85% (+0.31%) флэт
  - java_util: 7.01% -> 6.39% (-0.62%) флэт
  - paletted: 6.41% -> 7.06% (+0.66%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
