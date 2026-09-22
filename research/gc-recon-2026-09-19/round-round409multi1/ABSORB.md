# absorb ROUND (round409multi1, run 35670197810, branch round-408-multi1, head f7d04d2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6642199 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 6642199 (поллов=5); TPS_exp=2.20; normalized=+32.0%
- GC: young=107, Full=9, total=19.4s, avg=167ms, max=2350ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109067 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.15% (+0.44%) флэт
  - broadphase: 15.66% -> 13.57% (-2.08%) спад
  - nav_ai: 14.16% -> 7.02% (-7.14%) спад
  - inside_volatile: 12.01% -> 11.88% (-0.13%) флэт
  - fastutil: 8.54% -> 6.48% (-2.06%) спад
  - java_util: 7.01% -> 7.13% (+0.12%) флэт
  - paletted: 6.41% -> 6.41% (+0.00%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
