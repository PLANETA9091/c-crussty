# absorb ROUND (430-a2-l8r2, run 35873154182, branch round-430-a2-l8r2, head 18151a9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6694821 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6694821 (поллов=5); TPS_exp=2.21; normalized=+8.7%
- GC: young=103, Full=9, total=19.5s, avg=174ms, max=2493ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102350 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.69% (-0.03%) флэт
  - broadphase: 15.66% -> 10.21% (-5.44%) спад
  - nav_ai: 14.16% -> 3.33% (-10.83%) спад
  - inside_volatile: 12.01% -> 12.52% (+0.51%) флэт
  - fastutil: 8.54% -> 6.65% (-1.88%) спад
  - java_util: 7.01% -> 7.56% (+0.55%) флэт
  - paletted: 6.41% -> 5.69% (-0.72%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
