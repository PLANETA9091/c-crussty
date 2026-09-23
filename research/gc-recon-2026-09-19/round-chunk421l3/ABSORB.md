# absorb ROUND (chunk421l3, run 35804838622, branch round-421-c-chunk-l3, head 600e3cc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6870042 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.05 @ 6870042 (поллов=6); TPS_exp=2.25; normalized=-8.7%
- GC: young=108, Full=9, total=22.5s, avg=192ms, max=2466ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116986 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.92% (-1.25%) спад
  - fluid: 16.72% -> 15.62% (-1.10%) спад
  - broadphase: 15.66% -> 15.50% (-0.15%) флэт
  - nav_ai: 14.16% -> 13.60% (-0.56%) флэт
  - inside_volatile: 12.01% -> 11.25% (-0.76%) флэт
  - fastutil: 8.54% -> 8.73% (+0.20%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.09% (-0.32%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
