# absorb ROUND (anchor458-35, run 36146463025, branch round-458-anchor-35, head 96cc270)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6660866 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6660866 (поллов=5); TPS_exp=2.20; normalized=+13.6%
- GC: young=104, Full=9, total=21.6s, avg=191ms, max=2712ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104993 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.22% (+0.50%) флэт
  - broadphase: 15.66% -> 9.46% (-6.20%) спад
  - nav_ai: 14.16% -> 3.16% (-11.01%) спад
  - inside_volatile: 12.01% -> 16.56% (+4.55%) РОСТ
  - fastutil: 8.54% -> 7.08% (-1.46%) спад
  - java_util: 7.01% -> 9.11% (+2.10%) РОСТ
  - paletted: 6.41% -> 5.95% (-0.46%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
