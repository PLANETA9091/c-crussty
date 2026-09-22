# absorb ROUND (round405aleg2, run 35659531401, branch round-405-a-l2, head d538a32)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7237353 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7237353 (поллов=5); TPS_exp=2.32; normalized=+11.9%
- GC: young=117, Full=9, total=24.4s, avg=194ms, max=2886ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113552 сэмплов (базлайн 115655)
  - items: 31.17% -> 31.07% (-0.10%) флэт
  - fluid: 16.72% -> 18.27% (+1.55%) РОСТ
  - broadphase: 15.66% -> 16.24% (+0.58%) флэт
  - nav_ai: 14.16% -> 13.92% (-0.24%) флэт
  - inside_volatile: 12.01% -> 12.51% (+0.51%) флэт
  - fastutil: 8.54% -> 9.24% (+0.70%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 6.63% (+0.22%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
