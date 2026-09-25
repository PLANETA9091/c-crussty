# absorb ROUND (451b-anchor-22, run 36073671641, branch round-451-anchor-22, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6985894 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6985894 (поллов=5); TPS_exp=2.27; normalized=-7.5%
- GC: young=115, Full=9, total=21.7s, avg=175ms, max=2427ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116564 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.76% (-1.41%) спад
  - fluid: 16.72% -> 16.34% (-0.38%) флэт
  - broadphase: 15.66% -> 15.04% (-0.62%) флэт
  - nav_ai: 14.16% -> 13.85% (-0.31%) флэт
  - inside_volatile: 12.01% -> 11.32% (-0.69%) флэт
  - fastutil: 8.54% -> 8.54% (+0.01%) флэт
  - java_util: 7.01% -> 6.47% (-0.54%) флэт
  - paletted: 6.41% -> 6.20% (-0.21%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
