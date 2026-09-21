# absorb ROUND (round406eleg1, run 35660646779, branch round-406-e-l1, head 0578583)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6905070 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6905070 (поллов=6); TPS_exp=2.25; normalized=+13.2%
- GC: young=110, Full=9, total=19.7s, avg=165ms, max=2311ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113276 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.55% (+3.38%) РОСТ
  - fluid: 16.72% -> 17.50% (+0.78%) флэт
  - broadphase: 15.66% -> 15.62% (-0.04%) флэт
  - nav_ai: 14.16% -> 8.50% (-5.67%) спад
  - inside_volatile: 12.01% -> 12.20% (+0.20%) флэт
  - fastutil: 8.54% -> 6.96% (-1.58%) спад
  - java_util: 7.01% -> 6.97% (-0.04%) флэт
  - paletted: 6.41% -> 6.51% (+0.10%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
