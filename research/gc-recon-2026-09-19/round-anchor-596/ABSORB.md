# absorb ROUND (anchor-596, run 36207372924, branch round-463-anchor-596, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8408724 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8408724 (поллов=5); TPS_exp=2.57; normalized=-2.7%
- GC: young=122, Full=10, total=22.2s, avg=168ms, max=2140ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113786 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.64% (-2.53%) спад
  - fluid: 16.72% -> 15.91% (-0.81%) флэт
  - broadphase: 15.66% -> 14.60% (-1.06%) спад
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 10.99% (-1.01%) спад
  - fastutil: 8.54% -> 9.04% (+0.50%) флэт
  - java_util: 7.01% -> 6.79% (-0.22%) флэт
  - paletted: 6.41% -> 6.45% (+0.05%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
