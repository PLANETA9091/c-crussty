# absorb ROUND (round409eleg2, run 35667342821, branch round-406-e-l2, head fb8dc77)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7090478 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7090478 (поллов=6); TPS_exp=2.29; normalized=+9.1%
- GC: young=112, Full=9, total=19.6s, avg=162ms, max=2231ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112667 сэмплов (базлайн 115655)
  - items: 31.17% -> 33.95% (+2.78%) РОСТ
  - fluid: 16.72% -> 17.14% (+0.43%) флэт
  - broadphase: 15.66% -> 15.36% (-0.29%) флэт
  - nav_ai: 14.16% -> 9.11% (-5.05%) спад
  - inside_volatile: 12.01% -> 11.85% (-0.16%) флэт
  - fastutil: 8.54% -> 7.42% (-1.12%) спад
  - java_util: 7.01% -> 7.27% (+0.25%) флэт
  - paletted: 6.41% -> 6.29% (-0.11%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
