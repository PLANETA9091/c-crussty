# absorb ROUND (a6-456, run 36112130961, branch round-456-anchor-6, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6410979 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6410979 (поллов=5); TPS_exp=2.15; normalized=-2.3%
- GC: young=106, Full=9, total=20.4s, avg=177ms, max=2411ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116111 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.90% (-1.27%) спад
  - fluid: 16.72% -> 15.85% (-0.86%) флэт
  - broadphase: 15.66% -> 15.88% (+0.22%) флэт
  - nav_ai: 14.16% -> 14.33% (+0.16%) флэт
  - inside_volatile: 12.01% -> 11.37% (-0.63%) флэт
  - fastutil: 8.54% -> 8.70% (+0.16%) флэт
  - java_util: 7.01% -> 6.55% (-0.46%) флэт
  - paletted: 6.41% -> 6.36% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
