# absorb ROUND (b2l2, run 35811228420, branch round-422-b-brain2-l2, head 290c214)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6473041 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6473041 (поллов=5); TPS_exp=2.16; normalized=+15.6%
- GC: young=4124, Full=10, total=41.8s, avg=10ms, max=2102ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110844 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.11% (-2.61%) спад
  - broadphase: 15.66% -> 14.16% (-1.50%) спад
  - nav_ai: 14.16% -> 5.71% (-8.45%) спад
  - inside_volatile: 12.01% -> 12.39% (+0.38%) флэт
  - fastutil: 8.54% -> 9.09% (+0.55%) флэт
  - java_util: 7.01% -> 7.66% (+0.65%) флэт
  - paletted: 6.41% -> 5.43% (-0.98%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
