# absorb ROUND (mg420d, run 35798063088, branch round-420-mgd, head 77b22b7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6982758 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.85 @ 6982758 (поллов=6); TPS_exp=2.27; normalized=+25.6%
- GC: young=1134, Full=10, total=27.7s, avg=24ms, max=2377ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107660 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.84% (-0.88%) флэт
  - broadphase: 15.66% -> 13.72% (-1.94%) спад
  - nav_ai: 14.16% -> 9.18% (-4.99%) спад
  - inside_volatile: 12.01% -> 12.26% (+0.26%) флэт
  - fastutil: 8.54% -> 7.60% (-0.94%) флэт
  - java_util: 7.01% -> 8.22% (+1.20%) РОСТ
  - paletted: 6.41% -> 5.59% (-0.82%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
