# absorb ROUND (round-449-anchor-2, run 36043743956, branch round-449-anchor-2, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7000979 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 7000979 (поллов=5); TPS_exp=2.27; normalized=-16.4%
- GC: young=110, Full=9, total=21.5s, avg=180ms, max=2720ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117044 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.07% (-1.10%) спад
  - fluid: 16.72% -> 15.95% (-0.76%) флэт
  - broadphase: 15.66% -> 15.55% (-0.11%) флэт
  - nav_ai: 14.16% -> 13.93% (-0.23%) флэт
  - inside_volatile: 12.01% -> 11.66% (-0.34%) флэт
  - fastutil: 8.54% -> 8.65% (+0.11%) флэт
  - java_util: 7.01% -> 7.07% (+0.06%) флэт
  - paletted: 6.41% -> 6.40% (-0.01%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
