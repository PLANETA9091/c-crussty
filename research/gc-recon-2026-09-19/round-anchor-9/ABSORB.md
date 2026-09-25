# absorb ROUND (anchor-9, run 36093465302, branch round-454-anchor-9, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6559527 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6559527 (поллов=5); TPS_exp=2.18; normalized=+0.9%
- GC: young=110, Full=9, total=23.5s, avg=197ms, max=2519ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114841 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.47% (-1.70%) спад
  - fluid: 16.72% -> 16.72% (+0.01%) флэт
  - broadphase: 15.66% -> 14.80% (-0.86%) флэт
  - nav_ai: 14.16% -> 13.13% (-1.03%) спад
  - inside_volatile: 12.01% -> 10.92% (-1.09%) спад
  - fastutil: 8.54% -> 8.80% (+0.27%) флэт
  - java_util: 7.01% -> 6.50% (-0.51%) флэт
  - paletted: 6.41% -> 7.13% (+0.73%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
