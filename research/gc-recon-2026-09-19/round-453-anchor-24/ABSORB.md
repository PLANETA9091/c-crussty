# absorb ROUND (453-anchor-24, run 36090401265, branch round-453-anchor-24, head 2cfe205)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7173133 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7173133 (поллов=6); TPS_exp=2.31; normalized=-4.7%
- GC: young=108, Full=9, total=21.5s, avg=184ms, max=2418ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116061 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.56% (-1.61%) спад
  - fluid: 16.72% -> 16.08% (-0.64%) флэт
  - broadphase: 15.66% -> 15.86% (+0.20%) флэт
  - nav_ai: 14.16% -> 14.31% (+0.14%) флэт
  - inside_volatile: 12.01% -> 11.12% (-0.89%) флэт
  - fastutil: 8.54% -> 8.98% (+0.45%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.36% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
