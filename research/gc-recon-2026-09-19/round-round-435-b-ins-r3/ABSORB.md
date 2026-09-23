# absorb ROUND (round-435-b-ins-r3, run 35924457905, branch round-435-b-ins-r3, head 92cd1e5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6979303 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6979303 (поллов=6); TPS_exp=2.27; normalized=+12.4%
- GC: young=103, Full=5, total=13.1s, avg=121ms, max=554ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103478 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.11% (-0.61%) флэт
  - broadphase: 15.66% -> 9.84% (-5.81%) спад
  - nav_ai: 14.16% -> 3.13% (-11.04%) спад
  - inside_volatile: 12.01% -> 16.31% (+4.31%) РОСТ
  - fastutil: 8.54% -> 6.21% (-2.33%) спад
  - java_util: 7.01% -> 8.33% (+1.32%) РОСТ
  - paletted: 6.41% -> 5.45% (-0.95%) флэт
  - players_packets: 0.01% -> 0.02% (+0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
