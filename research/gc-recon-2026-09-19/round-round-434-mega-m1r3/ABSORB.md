# absorb ROUND (round-434-mega-m1r3, run 35919270034, branch round-434-mega-m1r3, head b15f329)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6564692 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6564692 (поллов=6); TPS_exp=2.18; normalized=+16.9%
- GC: young=102, Full=9, total=18.8s, avg=169ms, max=2422ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102135 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.26% (-0.45%) флэт
  - broadphase: 15.66% -> 10.08% (-5.58%) спад
  - nav_ai: 14.16% -> 3.31% (-10.85%) спад
  - inside_volatile: 12.01% -> 12.78% (+0.78%) флэт
  - fastutil: 8.54% -> 6.68% (-1.86%) спад
  - java_util: 7.01% -> 7.49% (+0.48%) флэт
  - paletted: 6.41% -> 5.43% (-0.98%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
