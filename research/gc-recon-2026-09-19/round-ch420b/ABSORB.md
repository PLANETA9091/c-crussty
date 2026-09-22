# absorb ROUND (ch420b, run 35790310095, branch round-420-c-chb, head 85f74b8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7097920 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 7097920 (поллов=6); TPS_exp=2.29; normalized=+6.8%
- GC: young=114, Full=9, total=21.1s, avg=171ms, max=2400ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116621 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.27% (-0.90%) флэт
  - fluid: 16.72% -> 16.05% (-0.67%) флэт
  - broadphase: 15.66% -> 14.81% (-0.84%) флэт
  - nav_ai: 14.16% -> 13.20% (-0.96%) флэт
  - inside_volatile: 12.01% -> 11.54% (-0.47%) флэт
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 6.16% (-0.25%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
