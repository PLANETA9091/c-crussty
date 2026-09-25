# absorb ROUND (a6, run 36104753399, branch round-455-anchor-6, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8395083 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8395083 (поллов=5); TPS_exp=2.57; normalized=+5.2%
- GC: young=123, Full=10, total=21.7s, avg=163ms, max=2120ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114214 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.57% (-2.60%) спад
  - fluid: 16.72% -> 16.68% (-0.03%) флэт
  - broadphase: 15.66% -> 15.44% (-0.22%) флэт
  - nav_ai: 14.16% -> 13.44% (-0.72%) флэт
  - inside_volatile: 12.01% -> 10.51% (-1.49%) спад
  - fastutil: 8.54% -> 8.46% (-0.08%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.90% (+0.49%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
