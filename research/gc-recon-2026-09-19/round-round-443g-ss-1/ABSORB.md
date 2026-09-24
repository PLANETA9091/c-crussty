# absorb ROUND (round-443g-ss-1, run 36042438517, branch round-443g-ss-1, head 3f3b111)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6706212 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6706212 (поллов=5); TPS_exp=2.21; normalized=-5.0%
- GC: young=113, Full=9, total=21.2s, avg=174ms, max=2389ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117141 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.46% (-1.72%) спад
  - fluid: 16.72% -> 15.95% (-0.76%) флэт
  - broadphase: 15.66% -> 15.18% (-0.48%) флэт
  - nav_ai: 14.16% -> 14.01% (-0.15%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.64%) флэт
  - fastutil: 8.54% -> 8.58% (+0.05%) флэт
  - java_util: 7.01% -> 6.64% (-0.37%) флэт
  - paletted: 6.41% -> 6.17% (-0.24%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
