# absorb ROUND (a22-456w2, run 36116737510, branch round-456-anchor-22, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6899724 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6899724 (поллов=6); TPS_exp=2.25; normalized=+4.4%
- GC: young=105, Full=9, total=20.2s, avg=178ms, max=2373ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116654 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.20% (-0.97%) флэт
  - fluid: 16.72% -> 15.98% (-0.74%) флэт
  - broadphase: 15.66% -> 15.62% (-0.04%) флэт
  - nav_ai: 14.16% -> 14.40% (+0.23%) флэт
  - inside_volatile: 12.01% -> 11.37% (-0.64%) флэт
  - fastutil: 8.54% -> 8.72% (+0.18%) флэт
  - java_util: 7.01% -> 6.53% (-0.49%) флэт
  - paletted: 6.41% -> 6.39% (-0.02%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
