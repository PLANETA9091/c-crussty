# absorb ROUND (round-442-anchor-9, run 35957120983, branch round-442-anchor-9, head 335f170)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8090856 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8090856 (поллов=5); TPS_exp=2.50; normalized=+3.9%
- GC: young=117, Full=9, total=21.5s, avg=171ms, max=2125ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113508 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.22% (-2.96%) спад
  - fluid: 16.72% -> 16.18% (-0.53%) флэт
  - broadphase: 15.66% -> 15.42% (-0.24%) флэт
  - nav_ai: 14.16% -> 14.54% (+0.37%) флэт
  - inside_volatile: 12.01% -> 10.21% (-1.80%) спад
  - fastutil: 8.54% -> 8.69% (+0.16%) флэт
  - java_util: 7.01% -> 6.42% (-0.59%) флэт
  - paletted: 6.41% -> 7.19% (+0.78%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
