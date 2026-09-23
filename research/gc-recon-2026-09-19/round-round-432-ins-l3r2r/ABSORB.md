# absorb ROUND (round-432-ins-l3r2r, run 35896489507, branch round-432-ins-l3r2r, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7612298 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 7612298 (поллов=5); TPS_exp=2.40; normalized=+29.1%
- GC: young=115, Full=9, total=22.5s, avg=181ms, max=3018ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104553 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.69% (+1.98%) РОСТ
  - broadphase: 15.66% -> 10.23% (-5.43%) спад
  - nav_ai: 14.16% -> 3.76% (-10.40%) спад
  - inside_volatile: 12.01% -> 16.03% (+4.03%) РОСТ
  - fastutil: 8.54% -> 6.47% (-2.07%) спад
  - java_util: 7.01% -> 7.86% (+0.85%) флэт
  - paletted: 6.41% -> 5.87% (-0.53%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
