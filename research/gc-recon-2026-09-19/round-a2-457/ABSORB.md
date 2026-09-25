# absorb ROUND (a2-457, run 36131584924, branch round-457-anchor-2, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6880646 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6880646 (поллов=5); TPS_exp=2.25; normalized=-2.1%
- GC: young=108, Full=10, total=24.1s, avg=204ms, max=2645ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116975 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.73% (-1.44%) спад
  - fluid: 16.72% -> 15.55% (-1.16%) спад
  - broadphase: 15.66% -> 14.65% (-1.00%) спад
  - nav_ai: 14.16% -> 13.35% (-0.82%) флэт
  - inside_volatile: 12.01% -> 11.86% (-0.14%) флэт
  - fastutil: 8.54% -> 8.74% (+0.20%) флэт
  - java_util: 7.01% -> 7.25% (+0.23%) флэт
  - paletted: 6.41% -> 6.13% (-0.28%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
