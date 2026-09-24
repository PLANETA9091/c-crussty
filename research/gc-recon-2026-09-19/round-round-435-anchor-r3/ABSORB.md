# absorb ROUND (round-435-anchor-r3, run 35924420253, branch round-435-anchor-r3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6436162 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6436162 (поллов=5); TPS_exp=2.15; normalized=+2.1%
- GC: young=113, Full=10, total=24.0s, avg=195ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116535 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.62% (-1.55%) спад
  - fluid: 16.72% -> 15.80% (-0.91%) флэт
  - broadphase: 15.66% -> 14.80% (-0.86%) флэт
  - nav_ai: 14.16% -> 13.70% (-0.46%) флэт
  - inside_volatile: 12.01% -> 11.65% (-0.36%) флэт
  - fastutil: 8.54% -> 8.91% (+0.37%) флэт
  - java_util: 7.01% -> 6.72% (-0.29%) флэт
  - paletted: 6.41% -> 6.29% (-0.12%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
