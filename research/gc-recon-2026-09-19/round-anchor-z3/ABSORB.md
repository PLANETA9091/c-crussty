# absorb ROUND (anchor-z3, run 35930440566, branch round-436-anchor-z3, head e05994c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6972473 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6972473 (поллов=6); TPS_exp=2.27; normalized=+3.7%
- GC: young=109, Full=9, total=20.7s, avg=175ms, max=2562ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117355 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.75% (-1.42%) спад
  - fluid: 16.72% -> 15.63% (-1.08%) спад
  - broadphase: 15.66% -> 15.12% (-0.54%) флэт
  - nav_ai: 14.16% -> 13.91% (-0.25%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.57%) флэт
  - fastutil: 8.54% -> 8.93% (+0.39%) флэт
  - java_util: 7.01% -> 6.44% (-0.57%) флэт
  - paletted: 6.41% -> 6.46% (+0.06%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
