# absorb ROUND (431c-anchor-c, run 35887006697, branch round-431c-anchor-c, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6997131 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6997131 (поллов=5); TPS_exp=2.27; normalized=-7.6%
- GC: young=113, Full=9, total=21.6s, avg=177ms, max=2405ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115878 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.46% (-1.71%) спад
  - fluid: 16.72% -> 15.87% (-0.84%) флэт
  - broadphase: 15.66% -> 15.07% (-0.59%) флэт
  - nav_ai: 14.16% -> 13.64% (-0.52%) флэт
  - inside_volatile: 12.01% -> 11.72% (-0.28%) флэт
  - fastutil: 8.54% -> 8.96% (+0.42%) флэт
  - java_util: 7.01% -> 6.50% (-0.51%) флэт
  - paletted: 6.41% -> 6.29% (-0.12%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
