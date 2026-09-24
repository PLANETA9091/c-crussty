# absorb ROUND (round-440-anchor-5, run 35950626864, branch round-440-anchor-5, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8513202 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 8513202 (поллов=6); TPS_exp=2.59; normalized=+6.1%
- GC: young=126, Full=9, total=24.9s, avg=185ms, max=2673ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=118008 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.10% (-2.07%) спад
  - fluid: 16.72% -> 16.68% (-0.04%) флэт
  - broadphase: 15.66% -> 13.82% (-1.84%) спад
  - nav_ai: 14.16% -> 13.14% (-1.02%) спад
  - inside_volatile: 12.01% -> 12.45% (+0.44%) флэт
  - fastutil: 8.54% -> 8.33% (-0.21%) флэт
  - java_util: 7.01% -> 6.79% (-0.22%) флэт
  - paletted: 6.41% -> 7.14% (+0.74%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
