# absorb ROUND (chkmono457-13, run 36143877485, branch round-456c-chunkmono-13, head d73758a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6831587 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6831587 (поллов=5); TPS_exp=2.24; normalized=+2.8%
- GC: young=102, Full=9, total=19.6s, avg=176ms, max=2586ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104695 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.27% (-0.44%) флэт
  - broadphase: 15.66% -> 9.67% (-5.98%) спад
  - nav_ai: 14.16% -> 3.44% (-10.72%) спад
  - inside_volatile: 12.01% -> 16.53% (+4.53%) РОСТ
  - fastutil: 8.54% -> 6.44% (-2.10%) спад
  - java_util: 7.01% -> 8.78% (+1.76%) РОСТ
  - paletted: 6.41% -> 5.06% (-1.35%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
