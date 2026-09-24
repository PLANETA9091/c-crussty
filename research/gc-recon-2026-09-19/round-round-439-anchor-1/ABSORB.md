# absorb ROUND (round-439-anchor-1, run 35946017071, branch round-439-anchor-1, head afbcde6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6922524 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6922524 (поллов=5); TPS_exp=2.26; normalized=-6.9%
- GC: young=106, Full=10, total=23.9s, avg=206ms, max=2603ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116177 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.00% (-1.17%) спад
  - fluid: 16.72% -> 15.80% (-0.91%) флэт
  - broadphase: 15.66% -> 15.71% (+0.06%) флэт
  - nav_ai: 14.16% -> 14.28% (+0.12%) флэт
  - inside_volatile: 12.01% -> 11.28% (-0.72%) флэт
  - fastutil: 8.54% -> 9.11% (+0.57%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 5.91% (-0.49%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
