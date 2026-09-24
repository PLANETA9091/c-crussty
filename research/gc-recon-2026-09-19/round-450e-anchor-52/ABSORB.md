# absorb ROUND (450e-anchor-52, run 36066599529, branch round-450-anchor-52, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6902871 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6902871 (поллов=5); TPS_exp=2.25; normalized=-6.8%
- GC: young=110, Full=9, total=21.1s, avg=178ms, max=2451ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116191 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.85% (-2.32%) спад
  - fluid: 16.72% -> 15.55% (-1.16%) спад
  - broadphase: 15.66% -> 14.70% (-0.95%) флэт
  - nav_ai: 14.16% -> 13.34% (-0.82%) флэт
  - inside_volatile: 12.01% -> 10.86% (-1.15%) спад
  - fastutil: 8.54% -> 8.26% (-0.28%) флэт
  - java_util: 7.01% -> 6.28% (-0.73%) флэт
  - paletted: 6.41% -> 6.10% (-0.31%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
