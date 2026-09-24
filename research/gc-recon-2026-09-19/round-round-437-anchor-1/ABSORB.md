# absorb ROUND (round-437-anchor-1, run 35935267948, branch round-437-anchor-1, head e4d39b1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6660905 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 6660905 (поллов=5); TPS_exp=2.20; normalized=-13.7%
- GC: young=112, Full=10, total=27.0s, avg=221ms, max=2613ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115935 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.70% (-1.47%) спад
  - fluid: 16.72% -> 16.98% (+0.26%) флэт
  - broadphase: 15.66% -> 14.87% (-0.79%) флэт
  - nav_ai: 14.16% -> 13.52% (-0.64%) флэт
  - inside_volatile: 12.01% -> 11.09% (-0.91%) флэт
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.27% (-0.74%) флэт
  - paletted: 6.41% -> 7.06% (+0.65%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
