# absorb ROUND (round-443g-anchor-9, run 36038673666, branch round-443g-anchor-9, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8985888 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 8985888 (поллов=5); TPS_exp=2.69; normalized=-7.1%
- GC: young=120, Full=10, total=22.1s, avg=170ms, max=2145ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112185 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.73% (-2.44%) спад
  - fluid: 16.72% -> 16.84% (+0.13%) флэт
  - broadphase: 15.66% -> 15.12% (-0.54%) флэт
  - nav_ai: 14.16% -> 14.13% (-0.03%) флэт
  - inside_volatile: 12.01% -> 10.41% (-1.59%) спад
  - fastutil: 8.54% -> 8.54% (+0.00%) флэт
  - java_util: 7.01% -> 6.86% (-0.16%) флэт
  - paletted: 6.41% -> 6.86% (+0.45%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
