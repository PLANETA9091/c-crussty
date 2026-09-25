# absorb ROUND (anchor-13, run 36097259500, branch round-454-anchor-13, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6919920 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6919920 (поллов=6); TPS_exp=2.26; normalized=+4.2%
- GC: young=111, Full=9, total=21.7s, avg=181ms, max=2759ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116716 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.43% (-1.74%) спад
  - fluid: 16.72% -> 15.36% (-1.35%) спад
  - broadphase: 15.66% -> 15.34% (-0.31%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.31%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.64%) флэт
  - fastutil: 8.54% -> 8.91% (+0.37%) флэт
  - java_util: 7.01% -> 6.59% (-0.42%) флэт
  - paletted: 6.41% -> 6.14% (-0.27%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
