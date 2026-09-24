# absorb ROUND (round-449a-collide-1, run 36025175644, branch round-449a-collide-1, head 7edda66)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7383497 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 7383497 (поллов=6); TPS_exp=2.35; normalized=+4.1%
- GC: young=103, Full=9, total=23.9s, avg=213ms, max=3228ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=101822 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 19.51% (+2.79%) РОСТ
  - broadphase: 15.66% -> 10.70% (-4.95%) спад
  - nav_ai: 14.16% -> 6.04% (-8.12%) спад
  - inside_volatile: 12.01% -> 12.71% (+0.70%) флэт
  - fastutil: 8.54% -> 5.45% (-3.08%) спад
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.47% (+0.07%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
