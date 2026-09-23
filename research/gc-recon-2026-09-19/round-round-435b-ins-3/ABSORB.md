# absorb ROUND (round-435b-ins-3, run 35926827459, branch round-435b-ins-3, head b85a8fc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6941436 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=36, TPS-поллов=6 -> **FAIL**
- T3: median=1.90 @ 6941436 (поллов=5); TPS_exp=2.26; normalized=-16.0%
- GC: young=108, Full=10, total=24.8s, avg=210ms, max=2689ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114429 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.77% (-0.40%) флэт
  - fluid: 16.72% -> 15.63% (-1.08%) спад
  - broadphase: 15.66% -> 14.29% (-1.37%) спад
  - nav_ai: 14.16% -> 12.65% (-1.51%) спад
  - inside_volatile: 12.01% -> 15.12% (+3.11%) РОСТ
  - fastutil: 8.54% -> 8.48% (-0.06%) флэт
  - java_util: 7.01% -> 7.47% (+0.45%) флэт
  - paletted: 6.41% -> 5.55% (-0.86%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **CRASH-REFUTED**
