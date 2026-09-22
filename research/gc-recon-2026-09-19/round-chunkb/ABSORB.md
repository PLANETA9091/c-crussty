# absorb ROUND (chunkb, run 35779695430, branch round-419-c-chb, head f7b0be4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8582258 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 8582258 (поллов=5); TPS_exp=2.61; normalized=+11.3%
- GC: young=126, Full=10, total=22.3s, avg=164ms, max=2131ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113179 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.44% (-2.73%) спад
  - fluid: 16.72% -> 16.37% (-0.34%) флэт
  - broadphase: 15.66% -> 15.06% (-0.60%) флэт
  - nav_ai: 14.16% -> 13.62% (-0.54%) флэт
  - inside_volatile: 12.01% -> 10.64% (-1.37%) спад
  - fastutil: 8.54% -> 8.74% (+0.20%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 6.81% (+0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
