# absorb ROUND (a22, run 36108670118, branch round-455-anchor-22, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7047888 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7047888 (поллов=5); TPS_exp=2.28; normalized=-8.0%
- GC: young=109, Full=9, total=21.0s, avg=178ms, max=2472ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116773 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.33% (-1.84%) спад
  - fluid: 16.72% -> 15.59% (-1.13%) спад
  - broadphase: 15.66% -> 15.23% (-0.43%) флэт
  - nav_ai: 14.16% -> 14.17% (+0.01%) флэт
  - inside_volatile: 12.01% -> 11.33% (-0.68%) флэт
  - fastutil: 8.54% -> 8.72% (+0.18%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.01% (-0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
