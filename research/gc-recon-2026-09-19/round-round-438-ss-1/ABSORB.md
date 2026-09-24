# absorb ROUND (round-438-ss-1, run 35941709132, branch round-438-ss-1, head 3f3b111)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8601275 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8601275 (поллов=5); TPS_exp=2.61; normalized=-8.1%
- GC: young=113, Full=10, total=27.4s, avg=223ms, max=3085ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117645 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.51% (-1.66%) спад
  - fluid: 16.72% -> 17.74% (+1.03%) РОСТ
  - broadphase: 15.66% -> 14.57% (-1.08%) спад
  - nav_ai: 14.16% -> 13.38% (-0.78%) флэт
  - inside_volatile: 12.01% -> 11.32% (-0.69%) флэт
  - fastutil: 8.54% -> 8.30% (-0.24%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 7.38% (+0.97%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
