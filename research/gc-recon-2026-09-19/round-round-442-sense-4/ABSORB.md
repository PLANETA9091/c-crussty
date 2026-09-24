# absorb ROUND (round-442-sense-4, run 35957202325, branch round-442-sense-4, head b6134c6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7231382 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7231382 (поллов=6); TPS_exp=2.32; normalized=+7.7%
- GC: young=110, Full=9, total=20.0s, avg=168ms, max=2301ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112243 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.80% (+3.63%) РОСТ
  - fluid: 16.72% -> 17.28% (+0.56%) флэт
  - broadphase: 15.66% -> 15.05% (-0.61%) флэт
  - nav_ai: 14.16% -> 8.35% (-5.81%) спад
  - inside_volatile: 12.01% -> 14.07% (+2.06%) РОСТ
  - fastutil: 8.54% -> 7.32% (-1.22%) спад
  - java_util: 7.01% -> 7.80% (+0.79%) флэт
  - paletted: 6.41% -> 6.15% (-0.25%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
