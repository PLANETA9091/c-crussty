# absorb ROUND (anchor-10, run 36093474493, branch round-454-anchor-10, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6832381 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6832381 (поллов=6); TPS_exp=2.24; normalized=+2.8%
- GC: young=110, Full=9, total=21.1s, avg=177ms, max=2564ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116600 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.70% (-1.47%) спад
  - fluid: 16.72% -> 15.81% (-0.91%) флэт
  - broadphase: 15.66% -> 15.58% (-0.07%) флэт
  - nav_ai: 14.16% -> 14.21% (+0.05%) флэт
  - inside_volatile: 12.01% -> 11.21% (-0.79%) флэт
  - fastutil: 8.54% -> 8.90% (+0.36%) флэт
  - java_util: 7.01% -> 6.69% (-0.32%) флэт
  - paletted: 6.41% -> 6.06% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
