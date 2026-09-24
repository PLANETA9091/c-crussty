# absorb ROUND (round-433-anchor-i2, run 35910342713, branch round-433-anchor-i2, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7069189 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=2.35 @ 7069189 (поллов=6); TPS_exp=2.29; normalized=+2.7%
- GC: young=112, Full=9, total=21.3s, avg=176ms, max=2752ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116582 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 15.73% (-0.98%) флэт
  - broadphase: 15.66% -> 15.37% (-0.29%) флэт
  - nav_ai: 14.16% -> 13.92% (-0.25%) флэт
  - inside_volatile: 12.01% -> 11.55% (-0.45%) флэт
  - fastutil: 8.54% -> 8.33% (-0.21%) флэт
  - java_util: 7.01% -> 6.71% (-0.30%) флэт
  - paletted: 6.41% -> 6.13% (-0.28%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **CRASH-REFUTED**
