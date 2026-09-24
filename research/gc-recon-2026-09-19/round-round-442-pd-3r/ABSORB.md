# absorb ROUND (round-442-pd-3r, run 35957193964, branch round-442-pd-3r, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7121055 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7121055 (поллов=5); TPS_exp=2.30; normalized=-8.6%
- GC: young=109, Full=9, total=21.2s, avg=179ms, max=2403ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117364 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.28% (-0.89%) флэт
  - fluid: 16.72% -> 16.16% (-0.56%) флэт
  - broadphase: 15.66% -> 15.87% (+0.21%) флэт
  - nav_ai: 14.16% -> 13.53% (-0.63%) флэт
  - inside_volatile: 12.01% -> 11.72% (-0.29%) флэт
  - fastutil: 8.54% -> 8.44% (-0.10%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 6.69% (+0.28%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
