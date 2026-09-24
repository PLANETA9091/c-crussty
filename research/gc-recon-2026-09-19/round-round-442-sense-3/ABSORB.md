# absorb ROUND (round-442-sense-3, run 35957145854, branch round-442-sense-3, head b6134c6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7176289 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7176289 (поллов=5); TPS_exp=2.31; normalized=-9.1%
- GC: young=102, Full=9, total=19.5s, avg=176ms, max=2704ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111428 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.84% (+3.66%) РОСТ
  - fluid: 16.72% -> 17.21% (+0.49%) флэт
  - broadphase: 15.66% -> 15.89% (+0.23%) флэт
  - nav_ai: 14.16% -> 8.19% (-5.97%) спад
  - inside_volatile: 12.01% -> 13.43% (+1.42%) РОСТ
  - fastutil: 8.54% -> 7.36% (-1.18%) спад
  - java_util: 7.01% -> 8.06% (+1.05%) РОСТ
  - paletted: 6.41% -> 5.94% (-0.46%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
