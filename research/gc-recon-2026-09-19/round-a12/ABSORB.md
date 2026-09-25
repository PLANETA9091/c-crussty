# absorb ROUND (a12, run 36104807197, branch round-455-anchor-12, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6429233 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.80 @ 6429233 (поллов=5); TPS_exp=2.15; normalized=-16.4%
- GC: young=108, Full=9, total=21.4s, avg=183ms, max=2533ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116257 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.79% (-1.38%) спад
  - fluid: 16.72% -> 15.82% (-0.90%) флэт
  - broadphase: 15.66% -> 15.06% (-0.60%) флэт
  - nav_ai: 14.16% -> 13.92% (-0.24%) флэт
  - inside_volatile: 12.01% -> 11.29% (-0.72%) флэт
  - fastutil: 8.54% -> 8.27% (-0.27%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
