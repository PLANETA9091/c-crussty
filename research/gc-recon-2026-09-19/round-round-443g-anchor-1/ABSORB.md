# absorb ROUND (round-443g-anchor-1, run 36038452618, branch round-443g-anchor-1, head b2e1993)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6891387 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6891387 (поллов=5); TPS_exp=2.25; normalized=-2.2%
- GC: young=108, Full=8, total=21.0s, avg=181ms, max=2446ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117117 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.00% (-1.17%) спад
  - fluid: 16.72% -> 15.77% (-0.94%) флэт
  - broadphase: 15.66% -> 14.96% (-0.70%) флэт
  - nav_ai: 14.16% -> 13.62% (-0.55%) флэт
  - inside_volatile: 12.01% -> 11.50% (-0.50%) флэт
  - fastutil: 8.54% -> 8.60% (+0.06%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.28% (-0.12%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
