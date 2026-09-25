# absorb ROUND (diet454-5, run 36097286848, branch round-454c-diet-5, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6708947 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6708947 (поллов=6); TPS_exp=2.21; normalized=+13.0%
- GC: young=106, Full=9, total=20.6s, avg=179ms, max=2990ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103054 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.00% (-0.71%) флэт
  - broadphase: 15.66% -> 9.94% (-5.72%) спад
  - nav_ai: 14.16% -> 3.38% (-10.78%) спад
  - inside_volatile: 12.01% -> 16.15% (+4.15%) РОСТ
  - fastutil: 8.54% -> 6.44% (-2.10%) спад
  - java_util: 7.01% -> 8.30% (+1.29%) РОСТ
  - paletted: 6.41% -> 5.23% (-1.17%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
