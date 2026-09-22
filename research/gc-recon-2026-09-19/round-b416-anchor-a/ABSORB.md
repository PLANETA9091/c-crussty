# absorb ROUND (b416-anchor-a, run 35745470489, branch round-416-anchora, head 5869010)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6776643 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6776643 (поллов=5); TPS_exp=2.23; normalized=-10.2%
- GC: young=105, Full=9, total=19.9s, avg=175ms, max=2428ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114721 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.80% (-0.37%) флэт
  - fluid: 16.72% -> 16.14% (-0.58%) флэт
  - broadphase: 15.66% -> 16.29% (+0.64%) флэт
  - nav_ai: 14.16% -> 13.99% (-0.18%) флэт
  - inside_volatile: 12.01% -> 11.78% (-0.23%) флэт
  - fastutil: 8.54% -> 8.57% (+0.04%) флэт
  - java_util: 7.01% -> 6.98% (-0.03%) флэт
  - paletted: 6.41% -> 6.26% (-0.15%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
