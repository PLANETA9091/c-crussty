# absorb ROUND (431b-anchor-c, run 35883963348, branch round-431b-anchor-c, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6853156 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6853156 (поллов=6); TPS_exp=2.24; normalized=+2.6%
- GC: young=113, Full=9, total=21.3s, avg=174ms, max=2501ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116343 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.23% (-0.94%) флэт
  - fluid: 16.72% -> 16.32% (-0.39%) флэт
  - broadphase: 15.66% -> 15.07% (-0.58%) флэт
  - nav_ai: 14.16% -> 13.64% (-0.52%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.58%) флэт
  - fastutil: 8.54% -> 8.42% (-0.12%) флэт
  - java_util: 7.01% -> 6.71% (-0.30%) флэт
  - paletted: 6.41% -> 6.39% (-0.02%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
