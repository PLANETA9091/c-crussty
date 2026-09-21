# absorb ROUND (round406anchorb, run 35640599131, branch round-406-anchorb, head ca5e1a4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6851475 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6851475 (поллов=5); TPS_exp=2.24; normalized=-6.3%
- GC: young=108, Full=9, total=21.3s, avg=182ms, max=2779ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115004 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.90% (-0.27%) флэт
  - fluid: 16.72% -> 16.60% (-0.12%) флэт
  - broadphase: 15.66% -> 15.96% (+0.30%) флэт
  - nav_ai: 14.16% -> 14.55% (+0.39%) флэт
  - inside_volatile: 12.01% -> 11.86% (-0.15%) флэт
  - fastutil: 8.54% -> 9.33% (+0.79%) флэт
  - java_util: 7.01% -> 7.04% (+0.03%) флэт
  - paletted: 6.41% -> 6.33% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
