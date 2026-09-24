# absorb ROUND (436c-anchor-1, run 35930819066, branch round-436c-anchor-1, head e05994c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7236789 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 7236789 (поллов=6); TPS_exp=2.32; normalized=+9.8%
- GC: young=113, Full=10, total=26.4s, avg=214ms, max=2886ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115997 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.41% (-0.76%) флэт
  - fluid: 16.72% -> 18.23% (+1.51%) РОСТ
  - broadphase: 15.66% -> 15.73% (+0.08%) флэт
  - nav_ai: 14.16% -> 14.20% (+0.03%) флэт
  - inside_volatile: 12.01% -> 11.63% (-0.38%) флэт
  - fastutil: 8.54% -> 9.03% (+0.49%) флэт
  - java_util: 7.01% -> 6.08% (-0.93%) флэт
  - paletted: 6.41% -> 6.44% (+0.03%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
