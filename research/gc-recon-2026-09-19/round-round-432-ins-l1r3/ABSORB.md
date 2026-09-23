# absorb ROUND (round-432-ins-l1r3, run 35891288141, branch round-432-ins-l1r3, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6804833 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 6804833 (поллов=6); TPS_exp=2.23; normalized=+25.5%
- GC: young=106, Full=9, total=19.2s, avg=167ms, max=2372ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103724 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.30% (-0.41%) флэт
  - broadphase: 15.66% -> 10.35% (-5.31%) спад
  - nav_ai: 14.16% -> 3.36% (-10.80%) спад
  - inside_volatile: 12.01% -> 14.96% (+2.96%) РОСТ
  - fastutil: 8.54% -> 6.98% (-1.55%) спад
  - java_util: 7.01% -> 8.65% (+1.64%) РОСТ
  - paletted: 6.41% -> 5.67% (-0.73%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
