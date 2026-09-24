# absorb ROUND (450e-anchor-49, run 36066565674, branch round-450-anchor-49, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6904829 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6904829 (поллов=5); TPS_exp=2.25; normalized=-6.8%
- GC: young=110, Full=8, total=24.3s, avg=206ms, max=2694ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115518 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.90% (-2.27%) спад
  - fluid: 16.72% -> 16.27% (-0.45%) флэт
  - broadphase: 15.66% -> 15.17% (-0.49%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.36%) флэт
  - inside_volatile: 12.01% -> 10.39% (-1.61%) спад
  - fastutil: 8.54% -> 8.58% (+0.04%) флэт
  - java_util: 7.01% -> 6.54% (-0.47%) флэт
  - paletted: 6.41% -> 6.84% (+0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
