# absorb ROUND (round-440-ss-1, run 35950609697, branch round-440-ss-1, head 3f3b111)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6824950 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6824950 (поллов=5); TPS_exp=2.24; normalized=+7.3%
- GC: young=112, Full=9, total=19.8s, avg=163ms, max=2397ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117380 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.92% (-1.25%) спад
  - fluid: 16.72% -> 16.10% (-0.62%) флэт
  - broadphase: 15.66% -> 14.48% (-1.18%) спад
  - nav_ai: 14.16% -> 13.63% (-0.53%) флэт
  - inside_volatile: 12.01% -> 11.51% (-0.50%) флэт
  - fastutil: 8.54% -> 8.96% (+0.42%) флэт
  - java_util: 7.01% -> 6.80% (-0.22%) флэт
  - paletted: 6.41% -> 6.30% (-0.11%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
