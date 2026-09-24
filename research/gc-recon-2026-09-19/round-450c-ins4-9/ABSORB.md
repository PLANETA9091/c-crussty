# absorb ROUND (450c-ins4-9, run 36058521648, branch round-450-ins4-9, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8907903 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.20 @ 8907903 (поллов=5); TPS_exp=2.68; normalized=+19.6%
- GC: young=123, Full=9, total=17.7s, avg=134ms, max=1931ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103544 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.37% (-0.34%) флэт
  - broadphase: 15.66% -> 8.92% (-6.74%) спад
  - nav_ai: 14.16% -> 4.01% (-10.16%) спад
  - inside_volatile: 12.01% -> 15.50% (+3.49%) РОСТ
  - fastutil: 8.54% -> 6.23% (-2.30%) спад
  - java_util: 7.01% -> 8.13% (+1.12%) РОСТ
  - paletted: 6.41% -> 6.04% (-0.37%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
