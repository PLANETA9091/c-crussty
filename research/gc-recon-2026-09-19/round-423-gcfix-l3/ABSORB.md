# absorb ROUND (round-423-gcfix-l3, run 35822171085, branch round-423-a-gcfix-l3, head 831394a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6624102 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6624102 (поллов=5); TPS_exp=2.19; normalized=+0.3%
- GC: young=4128, Full=11, total=44.9s, avg=11ms, max=2567ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110874 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.13% (-1.59%) спад
  - broadphase: 15.66% -> 15.48% (-0.18%) флэт
  - nav_ai: 14.16% -> 5.90% (-8.26%) спад
  - inside_volatile: 12.01% -> 12.07% (+0.06%) флэт
  - fastutil: 8.54% -> 9.58% (+1.04%) РОСТ
  - java_util: 7.01% -> 7.79% (+0.77%) флэт
  - paletted: 6.41% -> 5.43% (-0.98%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
