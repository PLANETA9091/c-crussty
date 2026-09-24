# absorb ROUND (450c-ins4-10, run 36058533463, branch round-450-ins4-10, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7177384 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7177384 (поллов=5); TPS_exp=2.31; normalized=+3.9%
- GC: young=104, Full=9, total=20.4s, avg=181ms, max=2502ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106640 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.27% (-0.44%) флэт
  - broadphase: 15.66% -> 9.54% (-6.12%) спад
  - nav_ai: 14.16% -> 3.73% (-10.44%) спад
  - inside_volatile: 12.01% -> 16.57% (+4.56%) РОСТ
  - fastutil: 8.54% -> 6.58% (-1.96%) спад
  - java_util: 7.01% -> 8.66% (+1.65%) РОСТ
  - paletted: 6.41% -> 5.21% (-1.19%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
