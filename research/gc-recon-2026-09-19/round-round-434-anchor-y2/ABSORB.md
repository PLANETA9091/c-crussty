# absorb ROUND (round-434-anchor-y2, run 35919382936, branch round-434-anchor-y2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6966629 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6966629 (поллов=5); TPS_exp=2.27; normalized=-7.3%
- GC: young=107, Full=9, total=21.0s, avg=181ms, max=2683ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116679 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.74% (-1.43%) спад
  - fluid: 16.72% -> 15.69% (-1.03%) спад
  - broadphase: 15.66% -> 15.09% (-0.57%) флэт
  - nav_ai: 14.16% -> 14.01% (-0.16%) флэт
  - inside_volatile: 12.01% -> 11.39% (-0.62%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 6.14% (-0.26%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
