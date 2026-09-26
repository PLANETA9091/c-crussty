# absorb ROUND (anchor-640, run 36206674786, branch round-463-anchor-640, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8808540 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8808540 (поллов=5); TPS_exp=2.65; normalized=+5.5%
- GC: young=126, Full=10, total=22.2s, avg=163ms, max=2089ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113544 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.28% (-1.89%) спад
  - fluid: 16.72% -> 16.83% (+0.11%) флэт
  - broadphase: 15.66% -> 14.49% (-1.17%) спад
  - nav_ai: 14.16% -> 13.68% (-0.49%) флэт
  - inside_volatile: 12.01% -> 10.70% (-1.31%) спад
  - fastutil: 8.54% -> 8.57% (+0.03%) флэт
  - java_util: 7.01% -> 6.28% (-0.73%) флэт
  - paletted: 6.41% -> 7.03% (+0.63%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
