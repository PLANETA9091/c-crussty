# absorb ROUND (mctrl, run 35730675117, branch round-414-mctrl, head f579c1c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6530239 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6530239 (поллов=5); TPS_exp=2.17; normalized=+19.6%
- GC: young=107, Full=9, total=18.1s, avg=156ms, max=2241ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107452 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.17% (-0.54%) флэт
  - broadphase: 15.66% -> 14.32% (-1.34%) спад
  - nav_ai: 14.16% -> 8.98% (-5.18%) спад
  - inside_volatile: 12.01% -> 12.11% (+0.10%) флэт
  - fastutil: 8.54% -> 7.32% (-1.22%) спад
  - java_util: 7.01% -> 7.59% (+0.58%) флэт
  - paletted: 6.41% -> 5.73% (-0.67%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
