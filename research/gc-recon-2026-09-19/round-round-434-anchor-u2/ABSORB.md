# absorb ROUND (round-434-anchor-u2, run 35919282510, branch round-434-anchor-u2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6842879 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6842879 (поллов=5); TPS_exp=2.24; normalized=-1.8%
- GC: young=113, Full=9, total=20.8s, avg=170ms, max=2411ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116499 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.42% (-1.75%) спад
  - fluid: 16.72% -> 16.21% (-0.50%) флэт
  - broadphase: 15.66% -> 15.13% (-0.52%) флэт
  - nav_ai: 14.16% -> 13.85% (-0.31%) флэт
  - inside_volatile: 12.01% -> 11.94% (-0.07%) флэт
  - fastutil: 8.54% -> 8.88% (+0.34%) флэт
  - java_util: 7.01% -> 6.33% (-0.68%) флэт
  - paletted: 6.41% -> 6.10% (-0.31%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
