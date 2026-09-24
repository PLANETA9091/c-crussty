# absorb ROUND (round-441-anchor-7, run 35954864895, branch round-441-anchor-7, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8541634 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8541634 (поллов=5); TPS_exp=2.60; normalized=+7.8%
- GC: young=126, Full=10, total=22.1s, avg=162ms, max=2095ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113255 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.76% (-2.41%) спад
  - fluid: 16.72% -> 16.45% (-0.27%) флэт
  - broadphase: 15.66% -> 15.17% (-0.48%) флэт
  - nav_ai: 14.16% -> 13.93% (-0.23%) флэт
  - inside_volatile: 12.01% -> 10.55% (-1.46%) спад
  - fastutil: 8.54% -> 8.99% (+0.46%) флэт
  - java_util: 7.01% -> 6.35% (-0.66%) флэт
  - paletted: 6.41% -> 6.81% (+0.41%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
