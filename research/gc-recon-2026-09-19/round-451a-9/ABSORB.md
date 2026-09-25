# absorb ROUND (451a-9, run 36069511063, branch round-451-anchor-9, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6452057 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6452057 (поллов=5); TPS_exp=2.16; normalized=-2.7%
- GC: young=112, Full=10, total=24.8s, avg=203ms, max=2588ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116811 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 16.01% (-0.70%) флэт
  - broadphase: 15.66% -> 14.88% (-0.78%) флэт
  - nav_ai: 14.16% -> 13.96% (-0.21%) флэт
  - inside_volatile: 12.01% -> 11.90% (-0.11%) флэт
  - fastutil: 8.54% -> 8.80% (+0.26%) флэт
  - java_util: 7.01% -> 6.21% (-0.80%) флэт
  - paletted: 6.41% -> 6.22% (-0.19%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
