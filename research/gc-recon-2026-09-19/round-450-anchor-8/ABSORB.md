# absorb ROUND (450-anchor-8, run 36050594456, branch round-450-anchor-8, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6552822 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6552822 (поллов=6); TPS_exp=2.18; normalized=+10.2%
- GC: young=109, Full=10, total=25.0s, avg=210ms, max=2623ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115622 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.94% (-2.23%) спад
  - fluid: 16.72% -> 16.58% (-0.14%) флэт
  - broadphase: 15.66% -> 15.01% (-0.65%) флэт
  - nav_ai: 14.16% -> 13.60% (-0.56%) флэт
  - inside_volatile: 12.01% -> 11.09% (-0.91%) флэт
  - fastutil: 8.54% -> 8.22% (-0.32%) флэт
  - java_util: 7.01% -> 8.08% (+1.07%) РОСТ
  - paletted: 6.41% -> 7.06% (+0.65%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
