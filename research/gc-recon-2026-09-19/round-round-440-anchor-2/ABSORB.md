# absorb ROUND (round-440-anchor-2, run 35950551638, branch round-440-anchor-2, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6723821 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6723821 (поллов=5); TPS_exp=2.21; normalized=-5.2%
- GC: young=108, Full=8, total=20.9s, avg=180ms, max=2380ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116400 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.57% (-1.60%) спад
  - fluid: 16.72% -> 15.70% (-1.02%) спад
  - broadphase: 15.66% -> 15.25% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.77% (-0.40%) флэт
  - inside_volatile: 12.01% -> 11.71% (-0.30%) флэт
  - fastutil: 8.54% -> 8.52% (-0.02%) флэт
  - java_util: 7.01% -> 6.67% (-0.34%) флэт
  - paletted: 6.41% -> 6.26% (-0.14%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
