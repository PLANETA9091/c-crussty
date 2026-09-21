# absorb ROUND (round403al2, run 35605934224, branch round-403-a-leg2, head c8bfbe0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=5204352 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 5204352 (поллов=6); TPS_exp=1.89; normalized=+31.9%
- GC: young=107, Full=9, total=19.0s, avg=164ms, max=2326ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110266 сэмплов (базлайн 115655)
  - items: 31.17% -> 36.00% (+4.83%) РОСТ
  - fluid: 16.72% -> 19.00% (+2.29%) РОСТ
  - broadphase: 15.66% -> 11.82% (-3.84%) спад
  - nav_ai: 14.16% -> 7.93% (-6.23%) спад
  - inside_volatile: 12.01% -> 12.53% (+0.53%) флэт
  - fastutil: 8.54% -> 7.81% (-0.72%) флэт
  - java_util: 7.01% -> 7.87% (+0.86%) флэт
  - paletted: 6.41% -> 6.58% (+0.18%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
