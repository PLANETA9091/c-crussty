# absorb ROUND (round-438-anchor-4, run 35941738591, branch round-438-anchor-4, head bcfb18f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6501644 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6501644 (поллов=6); TPS_exp=2.17; normalized=+13.0%
- GC: young=113, Full=9, total=23.0s, avg=188ms, max=2602ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115632 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.86% (-1.31%) спад
  - fluid: 16.72% -> 16.77% (+0.05%) флэт
  - broadphase: 15.66% -> 14.78% (-0.88%) флэт
  - nav_ai: 14.16% -> 13.66% (-0.51%) флэт
  - inside_volatile: 12.01% -> 11.11% (-0.90%) флэт
  - fastutil: 8.54% -> 9.03% (+0.50%) флэт
  - java_util: 7.01% -> 6.49% (-0.53%) флэт
  - paletted: 6.41% -> 6.59% (+0.19%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
