# absorb ROUND (round-423-cwgen-l1, run 35817966903, branch round-423-c-wgen-l1, head b582bc3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=5804616 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 5804616 (поллов=5); TPS_exp=2.02; normalized=+8.8%
- GC: young=117, Full=10, total=25.1s, avg=198ms, max=2459ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117591 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.76% (-1.41%) спад
  - fluid: 16.72% -> 15.67% (-1.05%) спад
  - broadphase: 15.66% -> 15.44% (-0.22%) флэт
  - nav_ai: 14.16% -> 14.08% (-0.09%) флэт
  - inside_volatile: 12.01% -> 11.58% (-0.42%) флэт
  - fastutil: 8.54% -> 8.81% (+0.28%) флэт
  - java_util: 7.01% -> 6.39% (-0.62%) флэт
  - paletted: 6.41% -> 5.98% (-0.42%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
