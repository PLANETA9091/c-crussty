# absorb ROUND (round-435c-anchor-4, run 35925949080, branch round-435c-anchor-4, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8814884 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8814884 (поллов=5); TPS_exp=2.66; normalized=-5.9%
- GC: young=120, Full=10, total=21.8s, avg=167ms, max=2149ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112952 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.50% (-2.67%) спад
  - fluid: 16.72% -> 16.58% (-0.13%) флэт
  - broadphase: 15.66% -> 15.17% (-0.49%) флэт
  - nav_ai: 14.16% -> 13.99% (-0.17%) флэт
  - inside_volatile: 12.01% -> 10.55% (-1.45%) спад
  - fastutil: 8.54% -> 9.28% (+0.75%) флэт
  - java_util: 7.01% -> 6.50% (-0.52%) флэт
  - paletted: 6.41% -> 6.89% (+0.48%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
