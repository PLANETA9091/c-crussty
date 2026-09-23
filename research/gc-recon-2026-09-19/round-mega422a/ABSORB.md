# absorb ROUND (mega422a, run 35809295484, branch round-422-mgc, head 99ffefa)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6753016 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6753016 (поллов=5); TPS_exp=2.22; normalized=+21.6%
- GC: young=1123, Full=9, total=25.6s, avg=23ms, max=2327ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108250 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.36% (-0.36%) флэт
  - broadphase: 15.66% -> 14.32% (-1.33%) спад
  - nav_ai: 14.16% -> 9.46% (-4.70%) спад
  - inside_volatile: 12.01% -> 12.71% (+0.71%) флэт
  - fastutil: 8.54% -> 8.12% (-0.42%) флэт
  - java_util: 7.01% -> 7.94% (+0.93%) флэт
  - paletted: 6.41% -> 5.88% (-0.53%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
