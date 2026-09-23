# absorb ROUND (round-434-anchor-r2, run 35914799077, branch round-434-anchor-r2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7217170 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7217170 (поллов=5); TPS_exp=2.32; normalized=-0.8%
- GC: young=119, Full=10, total=29.3s, avg=227ms, max=2937ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117243 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.46% (-0.71%) флэт
  - fluid: 16.72% -> 17.94% (+1.22%) РОСТ
  - broadphase: 15.66% -> 15.40% (-0.26%) флэт
  - nav_ai: 14.16% -> 13.77% (-0.39%) флэт
  - inside_volatile: 12.01% -> 12.45% (+0.45%) флэт
  - fastutil: 8.54% -> 8.75% (+0.22%) флэт
  - java_util: 7.01% -> 6.06% (-0.95%) флэт
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
