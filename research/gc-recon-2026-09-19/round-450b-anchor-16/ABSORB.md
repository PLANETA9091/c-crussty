# absorb ROUND (450b-anchor-16, run 36055326583, branch round-450-anchor-16, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8701361 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8701361 (поллов=5); TPS_exp=2.63; normalized=+6.4%
- GC: young=126, Full=10, total=21.9s, avg=161ms, max=2118ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113709 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.12% (-2.05%) спад
  - fluid: 16.72% -> 16.82% (+0.10%) флэт
  - broadphase: 15.66% -> 14.71% (-0.95%) флэт
  - nav_ai: 14.16% -> 13.70% (-0.47%) флэт
  - inside_volatile: 12.01% -> 11.00% (-1.00%) спад
  - fastutil: 8.54% -> 8.62% (+0.09%) флэт
  - java_util: 7.01% -> 6.41% (-0.60%) флэт
  - paletted: 6.41% -> 7.12% (+0.71%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
