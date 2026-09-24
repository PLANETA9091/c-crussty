# absorb ROUND (round-434-anchor-m2, run 35914658891, branch round-434-anchor-m2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7984332 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7984332 (поллов=5); TPS_exp=2.48; normalized=-7.3%
- GC: young=119, Full=9, total=25.1s, avg=196ms, max=3226ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117110 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.19% (-1.98%) спад
  - fluid: 16.72% -> 18.23% (+1.52%) РОСТ
  - broadphase: 15.66% -> 14.95% (-0.71%) флэт
  - nav_ai: 14.16% -> 13.37% (-0.79%) флэт
  - inside_volatile: 12.01% -> 11.51% (-0.49%) флэт
  - fastutil: 8.54% -> 8.47% (-0.07%) флэт
  - java_util: 7.01% -> 6.53% (-0.49%) флэт
  - paletted: 6.41% -> 7.42% (+1.02%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
