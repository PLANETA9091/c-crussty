# absorb ROUND (450d-anchor-29, run 36063311723, branch round-450-anchor-29, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6942256 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6942256 (поллов=5); TPS_exp=2.26; normalized=+6.2%
- GC: young=111, Full=9, total=19.6s, avg=163ms, max=2339ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116471 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.78% (-1.39%) спад
  - fluid: 16.72% -> 15.85% (-0.87%) флэт
  - broadphase: 15.66% -> 14.98% (-0.67%) флэт
  - nav_ai: 14.16% -> 14.10% (-0.06%) флэт
  - inside_volatile: 12.01% -> 11.77% (-0.23%) флэт
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.44% (-0.57%) флэт
  - paletted: 6.41% -> 6.20% (-0.21%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
