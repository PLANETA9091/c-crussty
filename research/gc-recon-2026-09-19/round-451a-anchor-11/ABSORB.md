# absorb ROUND (451a-anchor-11, run 36069530964, branch round-451-anchor-11, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6752503 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6752503 (поллов=6); TPS_exp=2.22; normalized=+3.6%
- GC: young=112, Full=9, total=21.3s, avg=176ms, max=2447ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116681 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.41% (-1.76%) спад
  - fluid: 16.72% -> 15.60% (-1.12%) спад
  - broadphase: 15.66% -> 15.59% (-0.07%) флэт
  - nav_ai: 14.16% -> 13.85% (-0.31%) флэт
  - inside_volatile: 12.01% -> 11.34% (-0.67%) флэт
  - fastutil: 8.54% -> 8.70% (+0.16%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 6.12% (-0.29%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
