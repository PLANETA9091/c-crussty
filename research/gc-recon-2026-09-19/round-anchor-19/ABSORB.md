# absorb ROUND (anchor-19, run 36101864640, branch round-454-anchor-19, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6830319 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6830319 (поллов=5); TPS_exp=2.24; normalized=-1.7%
- GC: young=111, Full=9, total=20.3s, avg=169ms, max=2468ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116948 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.09% (-1.08%) спад
  - fluid: 16.72% -> 16.07% (-0.65%) флэт
  - broadphase: 15.66% -> 15.66% (+0.00%) флэт
  - nav_ai: 14.16% -> 13.70% (-0.46%) флэт
  - inside_volatile: 12.01% -> 11.45% (-0.56%) флэт
  - fastutil: 8.54% -> 9.04% (+0.50%) флэт
  - java_util: 7.01% -> 6.49% (-0.52%) флэт
  - paletted: 6.41% -> 6.21% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
