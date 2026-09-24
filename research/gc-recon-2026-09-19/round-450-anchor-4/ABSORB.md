# absorb ROUND (450-anchor-4, run 36050539651, branch round-450-anchor-4, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6725322 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6725322 (поллов=5); TPS_exp=2.22; normalized=+3.8%
- GC: young=114, Full=9, total=20.8s, avg=170ms, max=2378ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116938 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.48% (-1.69%) спад
  - fluid: 16.72% -> 15.55% (-1.17%) спад
  - broadphase: 15.66% -> 15.86% (+0.20%) флэт
  - nav_ai: 14.16% -> 13.84% (-0.32%) флэт
  - inside_volatile: 12.01% -> 11.50% (-0.51%) флэт
  - fastutil: 8.54% -> 8.91% (+0.37%) флэт
  - java_util: 7.01% -> 6.68% (-0.33%) флэт
  - paletted: 6.41% -> 6.16% (-0.24%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
