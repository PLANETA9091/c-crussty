# absorb ROUND (a26-456w2, run 36116782703, branch round-456-anchor-26, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6572707 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6572707 (поллов=5); TPS_exp=2.18; normalized=-3.8%
- GC: young=108, Full=9, total=21.2s, avg=181ms, max=2407ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117866 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.23% (-1.94%) спад
  - fluid: 16.72% -> 15.20% (-1.52%) спад
  - broadphase: 15.66% -> 14.51% (-1.15%) спад
  - nav_ai: 14.16% -> 13.38% (-0.78%) флэт
  - inside_volatile: 12.01% -> 11.27% (-0.73%) флэт
  - fastutil: 8.54% -> 8.29% (-0.25%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 6.06% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
