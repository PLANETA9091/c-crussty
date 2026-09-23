# absorb ROUND (round-428-anchord, run 35856034466, branch round-428-anchord, head dc704c9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6964469 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6964469 (поллов=6); TPS_exp=2.27; normalized=+8.1%
- GC: young=111, Full=9, total=20.8s, avg=173ms, max=2362ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116928 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.37% (-1.80%) спад
  - fluid: 16.72% -> 15.62% (-1.10%) спад
  - broadphase: 15.66% -> 15.57% (-0.09%) флэт
  - nav_ai: 14.16% -> 14.04% (-0.12%) флэт
  - inside_volatile: 12.01% -> 11.24% (-0.76%) флэт
  - fastutil: 8.54% -> 8.58% (+0.05%) флэт
  - java_util: 7.01% -> 6.71% (-0.30%) флэт
  - paletted: 6.41% -> 6.03% (-0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
