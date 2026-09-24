# absorb ROUND (451a-anchor-3, run 36069448672, branch round-451-anchor-3, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7130856 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7130856 (поллов=6); TPS_exp=2.30; normalized=-0.0%
- GC: young=113, Full=9, total=21.8s, avg=179ms, max=2665ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116981 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 15.58% (-1.13%) спад
  - broadphase: 15.66% -> 15.43% (-0.23%) флэт
  - nav_ai: 14.16% -> 13.96% (-0.20%) флэт
  - inside_volatile: 12.01% -> 11.76% (-0.24%) флэт
  - fastutil: 8.54% -> 9.09% (+0.55%) флэт
  - java_util: 7.01% -> 6.89% (-0.12%) флэт
  - paletted: 6.41% -> 6.13% (-0.28%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
