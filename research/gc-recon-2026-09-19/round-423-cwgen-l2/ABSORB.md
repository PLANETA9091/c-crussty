# absorb ROUND (round-423-cwgen-l2, run 35817971411, branch round-423-c-wgen-l2, head b582bc3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7536998 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7536998 (поллов=5); TPS_exp=2.39; normalized=+0.6%
- GC: young=119, Full=9, total=25.6s, avg=200ms, max=2941ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117166 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.24% (-0.93%) флэт
  - fluid: 16.72% -> 18.01% (+1.29%) РОСТ
  - broadphase: 15.66% -> 15.93% (+0.27%) флэт
  - nav_ai: 14.16% -> 13.88% (-0.28%) флэт
  - inside_volatile: 12.01% -> 12.29% (+0.28%) флэт
  - fastutil: 8.54% -> 7.99% (-0.55%) флэт
  - java_util: 7.01% -> 5.86% (-1.15%) спад
  - paletted: 6.41% -> 6.20% (-0.21%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**

## BOOT-METRIC (TASK-425-C, player-visible chunk axis)
- boot_done_s: **16.678s**
- ramp_polls (first 3 soak): [1.8, 2.1, 2.4]
- tps_med (all soak polls): 2.4
