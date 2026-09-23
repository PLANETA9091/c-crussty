# absorb ROUND (round-428-anchora, run 35853592336, branch round-428-anchora, head dc704c9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6717950 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6717950 (поллов=6); TPS_exp=2.21; normalized=+3.9%
- GC: young=110, Full=8, total=21.4s, avg=181ms, max=2500ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117128 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.20% (-0.97%) флэт
  - fluid: 16.72% -> 16.44% (-0.28%) флэт
  - broadphase: 15.66% -> 15.26% (-0.40%) флэт
  - nav_ai: 14.16% -> 14.14% (-0.02%) флэт
  - inside_volatile: 12.01% -> 11.74% (-0.27%) флэт
  - fastutil: 8.54% -> 8.66% (+0.13%) флэт
  - java_util: 7.01% -> 6.71% (-0.31%) флэт
  - paletted: 6.41% -> 6.10% (-0.30%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
