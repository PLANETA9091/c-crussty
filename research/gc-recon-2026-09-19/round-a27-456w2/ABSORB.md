# absorb ROUND (a27-456w2, run 36116794291, branch round-456-anchor-27, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6125089 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6125089 (поллов=5); TPS_exp=2.09; normalized=+10.1%
- GC: young=115, Full=9, total=23.2s, avg=187ms, max=2798ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116169 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.20% (-1.97%) спад
  - fluid: 16.72% -> 16.61% (-0.10%) флэт
  - broadphase: 15.66% -> 14.61% (-1.05%) спад
  - nav_ai: 14.16% -> 13.56% (-0.60%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.64%) флэт
  - fastutil: 8.54% -> 8.92% (+0.38%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.97% (+0.57%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
