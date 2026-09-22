# absorb ROUND (round412br1, run 35708224953, branch round-412-b-r1, head 5954122)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6933935 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6933935 (поллов=6); TPS_exp=2.26; normalized=+4.0%
- GC: young=106, Full=9, total=20.3s, avg=177ms, max=2471ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114461 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.89% (-0.28%) флэт
  - fluid: 16.72% -> 16.59% (-0.12%) флэт
  - broadphase: 15.66% -> 15.75% (+0.09%) флэт
  - nav_ai: 14.16% -> 14.68% (+0.52%) флэт
  - inside_volatile: 12.01% -> 11.73% (-0.28%) флэт
  - fastutil: 8.54% -> 8.88% (+0.34%) флэт
  - java_util: 7.01% -> 6.97% (-0.04%) флэт
  - paletted: 6.41% -> 6.63% (+0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
