# absorb ROUND (round-439-anchor-3, run 35946050102, branch round-439-anchor-3, head afbcde6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6952536 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6952536 (поллов=5); TPS_exp=2.26; normalized=-7.2%
- GC: young=110, Full=10, total=24.7s, avg=206ms, max=2372ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116762 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.64% (-1.53%) спад
  - fluid: 16.72% -> 15.84% (-0.87%) флэт
  - broadphase: 15.66% -> 14.76% (-0.90%) флэт
  - nav_ai: 14.16% -> 13.73% (-0.43%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.65%) флэт
  - fastutil: 8.54% -> 8.88% (+0.34%) флэт
  - java_util: 7.01% -> 6.26% (-0.75%) флэт
  - paletted: 6.41% -> 6.36% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
