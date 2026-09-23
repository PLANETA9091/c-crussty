# absorb ROUND (round-434-anchor-v2, run 35919307265, branch round-434-anchor-v2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7013562 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7013562 (поллов=5); TPS_exp=2.28; normalized=+5.5%
- GC: young=119, Full=10, total=24.6s, avg=191ms, max=2394ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116689 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.11% (-2.06%) спад
  - fluid: 16.72% -> 15.54% (-1.18%) спад
  - broadphase: 15.66% -> 14.27% (-1.39%) спад
  - nav_ai: 14.16% -> 13.74% (-0.42%) флэт
  - inside_volatile: 12.01% -> 11.90% (-0.11%) флэт
  - fastutil: 8.54% -> 8.80% (+0.27%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.16% (-0.25%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
