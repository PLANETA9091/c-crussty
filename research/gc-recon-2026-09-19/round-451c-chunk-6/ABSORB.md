# absorb ROUND (451c-chunk-6, run 36070481536, branch round-451c-chunk-6, head 29ac648)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6751706 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6751706 (поллов=5); TPS_exp=2.22; normalized=+12.6%
- GC: young=109, Full=9, total=20.2s, avg=171ms, max=2961ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104061 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.40% (-0.31%) флэт
  - broadphase: 15.66% -> 9.58% (-6.08%) спад
  - nav_ai: 14.16% -> 3.18% (-10.98%) спад
  - inside_volatile: 12.01% -> 16.38% (+4.38%) РОСТ
  - fastutil: 8.54% -> 6.69% (-1.85%) спад
  - java_util: 7.01% -> 8.58% (+1.57%) РОСТ
  - paletted: 6.41% -> 5.35% (-1.06%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
