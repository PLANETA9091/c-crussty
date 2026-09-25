# absorb ROUND (a18-456w2, run 36116691170, branch round-456-anchor-18, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6941789 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6941789 (поллов=5); TPS_exp=2.26; normalized=-11.5%
- GC: young=111, Full=10, total=24.1s, avg=200ms, max=2458ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116820 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.20% (-1.97%) спад
  - fluid: 16.72% -> 15.71% (-1.01%) спад
  - broadphase: 15.66% -> 16.04% (+0.38%) флэт
  - nav_ai: 14.16% -> 14.29% (+0.13%) флэт
  - inside_volatile: 12.01% -> 11.23% (-0.77%) флэт
  - fastutil: 8.54% -> 8.57% (+0.04%) флэт
  - java_util: 7.01% -> 6.33% (-0.68%) флэт
  - paletted: 6.41% -> 5.97% (-0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
