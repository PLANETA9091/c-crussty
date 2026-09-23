# absorb ROUND (431-a2-l9r4, run 35880139012, branch round-431-a2-l9r4, head 18151a9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6978629 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6978629 (поллов=5); TPS_exp=2.27; normalized=+19.0%
- GC: young=106, Full=9, total=19.8s, avg=172ms, max=2868ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103577 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.79% (+0.07%) флэт
  - broadphase: 15.66% -> 10.25% (-5.40%) спад
  - nav_ai: 14.16% -> 3.37% (-10.79%) спад
  - inside_volatile: 12.01% -> 13.12% (+1.11%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.20%) спад
  - java_util: 7.01% -> 7.31% (+0.30%) флэт
  - paletted: 6.41% -> 5.74% (-0.67%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
