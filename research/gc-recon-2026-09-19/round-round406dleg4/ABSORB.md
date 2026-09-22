# absorb ROUND (round406dleg4, run 35659164007, branch round-406-d-l4, head dac1140)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7043848 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 7043848 (поллов=6); TPS_exp=2.28; normalized=+22.7%
- GC: young=110, Full=9, total=18.6s, avg=157ms, max=2279ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111349 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.22% (-0.50%) флэт
  - broadphase: 15.66% -> 14.27% (-1.38%) спад
  - nav_ai: 14.16% -> 9.64% (-4.52%) спад
  - inside_volatile: 12.01% -> 12.33% (+0.32%) флэт
  - fastutil: 8.54% -> 7.34% (-1.20%) спад
  - java_util: 7.01% -> 7.23% (+0.22%) флэт
  - paletted: 6.41% -> 5.80% (-0.60%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
