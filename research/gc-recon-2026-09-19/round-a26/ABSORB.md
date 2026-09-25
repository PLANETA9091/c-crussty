# absorb ROUND (a26, run 36108712345, branch round-455-anchor-26, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8854219 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 8854219 (поллов=5); TPS_exp=2.66; normalized=-17.4%
- GC: young=108, Full=10, total=25.2s, avg=213ms, max=2871ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117545 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.83% (-1.34%) спад
  - fluid: 16.72% -> 17.59% (+0.87%) флэт
  - broadphase: 15.66% -> 14.89% (-0.76%) флэт
  - nav_ai: 14.16% -> 13.23% (-0.93%) флэт
  - inside_volatile: 12.01% -> 11.55% (-0.45%) флэт
  - fastutil: 8.54% -> 8.55% (+0.01%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 7.36% (+0.96%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
