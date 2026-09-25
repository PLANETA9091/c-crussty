# absorb ROUND (anchor-16, run 36101839493, branch round-454-anchor-16, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7066543 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 7066543 (поллов=5); TPS_exp=2.29; normalized=-16.9%
- GC: young=109, Full=9, total=20.8s, avg=177ms, max=2332ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117127 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.99% (-2.18%) спад
  - fluid: 16.72% -> 15.16% (-1.56%) спад
  - broadphase: 15.66% -> 14.89% (-0.77%) флэт
  - nav_ai: 14.16% -> 13.50% (-0.66%) флэт
  - inside_volatile: 12.01% -> 11.10% (-0.91%) флэт
  - fastutil: 8.54% -> 8.37% (-0.17%) флэт
  - java_util: 7.01% -> 6.68% (-0.33%) флэт
  - paletted: 6.41% -> 5.96% (-0.44%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
