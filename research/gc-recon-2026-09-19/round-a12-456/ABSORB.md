# absorb ROUND (a12-456, run 36112193920, branch round-456-anchor-12, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6966170 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6966170 (поллов=6); TPS_exp=2.27; normalized=+8.1%
- GC: young=110, Full=9, total=20.9s, avg=176ms, max=2641ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116651 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.83% (-1.34%) спад
  - fluid: 16.72% -> 15.96% (-0.76%) флэт
  - broadphase: 15.66% -> 15.16% (-0.50%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 11.76% (-0.25%) флэт
  - fastutil: 8.54% -> 8.78% (+0.24%) флэт
  - java_util: 7.01% -> 6.90% (-0.11%) флэт
  - paletted: 6.41% -> 6.31% (-0.10%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
