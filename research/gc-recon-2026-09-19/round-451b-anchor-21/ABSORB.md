# absorb ROUND (451b-anchor-21, run 36073662164, branch round-451-anchor-21, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6616317 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.40 @ 6616317 (поллов=5); TPS_exp=2.19; normalized=-36.1%
- GC: young=92, Full=9, total=28.2s, avg=279ms, max=3635ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116822 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.99% (-1.18%) спад
  - fluid: 16.72% -> 15.93% (-0.78%) флэт
  - broadphase: 15.66% -> 15.48% (-0.17%) флэт
  - nav_ai: 14.16% -> 13.06% (-1.10%) спад
  - inside_volatile: 12.01% -> 11.74% (-0.27%) флэт
  - fastutil: 8.54% -> 8.41% (-0.12%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.02% (-0.38%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
