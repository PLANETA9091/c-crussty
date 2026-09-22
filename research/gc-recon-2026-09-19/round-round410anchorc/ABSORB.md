# absorb ROUND (round410anchorc, run 35673627761, branch round-410-anchorc, head 8ef9e5b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8827927 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8827927 (поллов=5); TPS_exp=2.66; normalized=-2.2%
- GC: young=121, Full=7, total=18.8s, avg=147ms, max=2141ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112082 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.18% (-1.99%) спад
  - fluid: 16.72% -> 17.15% (+0.43%) флэт
  - broadphase: 15.66% -> 15.48% (-0.18%) флэт
  - nav_ai: 14.16% -> 13.90% (-0.27%) флэт
  - inside_volatile: 12.01% -> 10.89% (-1.12%) спад
  - fastutil: 8.54% -> 8.97% (+0.43%) флэт
  - java_util: 7.01% -> 6.82% (-0.20%) флэт
  - paletted: 6.41% -> 7.16% (+0.76%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
