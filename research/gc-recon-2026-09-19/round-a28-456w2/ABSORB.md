# absorb ROUND (a28-456w2, run 36116806512, branch round-456-anchor-28, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8746488 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8746488 (поллов=5); TPS_exp=2.64; normalized=-9.1%
- GC: young=123, Full=10, total=22.6s, avg=170ms, max=2206ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112238 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.67% (-2.50%) спад
  - fluid: 16.72% -> 16.59% (-0.13%) флэт
  - broadphase: 15.66% -> 15.34% (-0.32%) флэт
  - nav_ai: 14.16% -> 14.04% (-0.13%) флэт
  - inside_volatile: 12.01% -> 10.72% (-1.29%) спад
  - fastutil: 8.54% -> 8.75% (+0.21%) флэт
  - java_util: 7.01% -> 6.44% (-0.57%) флэт
  - paletted: 6.41% -> 6.85% (+0.45%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
