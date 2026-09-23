# absorb ROUND (431-a2-l3r4, run 35880107270, branch round-431-a2-l3r4, head 18151a9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6744145 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6744145 (поллов=5); TPS_exp=2.22; normalized=+8.2%
- GC: young=102, Full=9, total=19.9s, avg=179ms, max=2646ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103602 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.49% (-0.23%) флэт
  - broadphase: 15.66% -> 10.47% (-5.19%) спад
  - nav_ai: 14.16% -> 3.47% (-10.70%) спад
  - inside_volatile: 12.01% -> 12.74% (+0.74%) флэт
  - fastutil: 8.54% -> 6.98% (-1.56%) спад
  - java_util: 7.01% -> 7.25% (+0.24%) флэт
  - paletted: 6.41% -> 5.47% (-0.94%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
