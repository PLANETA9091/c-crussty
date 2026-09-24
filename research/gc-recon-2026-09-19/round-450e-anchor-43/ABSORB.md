# absorb ROUND (450e-anchor-43, run 36066495171, branch round-450-anchor-43, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9036595 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 9036595 (поллов=5); TPS_exp=2.70; normalized=-0.1%
- GC: young=124, Full=10, total=22.5s, avg=168ms, max=2132ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112276 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.46% (-2.72%) спад
  - fluid: 16.72% -> 16.59% (-0.13%) флэт
  - broadphase: 15.66% -> 15.03% (-0.63%) флэт
  - nav_ai: 14.16% -> 14.13% (-0.03%) флэт
  - inside_volatile: 12.01% -> 10.60% (-1.41%) спад
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.52% (-0.49%) флэт
  - paletted: 6.41% -> 6.83% (+0.42%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
