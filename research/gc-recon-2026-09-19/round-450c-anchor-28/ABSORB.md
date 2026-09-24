# absorb ROUND (450c-anchor-28, run 36058481399, branch round-450-anchor-28, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6554643 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 6554643 (поллов=6); TPS_exp=2.18; normalized=+21.6%
- GC: young=112, Full=8, total=24.1s, avg=201ms, max=2957ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116354 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.47% (-0.70%) флэт
  - fluid: 16.72% -> 18.42% (+1.70%) РОСТ
  - broadphase: 15.66% -> 15.73% (+0.07%) флэт
  - nav_ai: 14.16% -> 14.06% (-0.10%) флэт
  - inside_volatile: 12.01% -> 11.75% (-0.25%) флэт
  - fastutil: 8.54% -> 8.98% (+0.44%) флэт
  - java_util: 7.01% -> 6.11% (-0.91%) флэт
  - paletted: 6.41% -> 6.44% (+0.03%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
