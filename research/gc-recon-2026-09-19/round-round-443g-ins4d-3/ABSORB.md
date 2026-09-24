# absorb ROUND (round-443g-ins4d-3, run 36043779900, branch round-443g-ins4d-3, head d9d1fb3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8985189 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=1, TPS-поллов=6 -> **FAIL**
- T3: median=3.30 @ 8985189 (поллов=5); TPS_exp=2.69; normalized=+22.6%
- GC: young=127, Full=10, total=21.2s, avg=155ms, max=2099ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103132 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.96% (-0.76%) флэт
  - broadphase: 15.66% -> 8.84% (-6.81%) спад
  - nav_ai: 14.16% -> 3.95% (-10.21%) спад
  - inside_volatile: 12.01% -> 15.55% (+3.54%) РОСТ
  - fastutil: 8.54% -> 6.32% (-2.22%) спад
  - java_util: 7.01% -> 7.98% (+0.96%) флэт
  - paletted: 6.41% -> 5.56% (-0.85%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **CRASH-REFUTED**
