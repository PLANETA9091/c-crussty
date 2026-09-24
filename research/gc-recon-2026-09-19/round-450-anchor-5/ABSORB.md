# absorb ROUND (450-anchor-5, run 36050553119, branch round-450-anchor-5, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8287902 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8287902 (поллов=5); TPS_exp=2.54; normalized=+6.1%
- GC: young=119, Full=10, total=21.4s, avg=166ms, max=2181ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113679 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.74% (-2.43%) спад
  - fluid: 16.72% -> 16.15% (-0.56%) флэт
  - broadphase: 15.66% -> 15.30% (-0.35%) флэт
  - nav_ai: 14.16% -> 13.84% (-0.32%) флэт
  - inside_volatile: 12.01% -> 10.53% (-1.48%) спад
  - fastutil: 8.54% -> 9.28% (+0.74%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.58% (+0.18%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
