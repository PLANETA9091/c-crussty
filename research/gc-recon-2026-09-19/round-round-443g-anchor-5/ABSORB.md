# absorb ROUND (round-443g-anchor-5, run 36043768180, branch round-443g-anchor-5, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7577717 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7577717 (поллов=5); TPS_exp=2.39; normalized=+4.4%
- GC: young=122, Full=10, total=23.0s, avg=174ms, max=2215ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113015 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.98% (-2.19%) спад
  - fluid: 16.72% -> 16.62% (-0.09%) флэт
  - broadphase: 15.66% -> 14.63% (-1.03%) спад
  - nav_ai: 14.16% -> 13.96% (-0.20%) флэт
  - inside_volatile: 12.01% -> 10.57% (-1.43%) спад
  - fastutil: 8.54% -> 9.40% (+0.86%) флэт
  - java_util: 7.01% -> 6.53% (-0.48%) флэт
  - paletted: 6.41% -> 6.93% (+0.52%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
