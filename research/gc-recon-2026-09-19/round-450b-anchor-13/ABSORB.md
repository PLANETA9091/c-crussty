# absorb ROUND (450b-anchor-13, run 36055279145, branch round-450-anchor-13, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6964170 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6964170 (поллов=6); TPS_exp=2.27; normalized=+5.9%
- GC: young=112, Full=9, total=20.3s, avg=168ms, max=2371ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117358 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.78% (-1.39%) спад
  - fluid: 16.72% -> 15.85% (-0.87%) флэт
  - broadphase: 15.66% -> 15.23% (-0.43%) флэт
  - nav_ai: 14.16% -> 13.88% (-0.28%) флэт
  - inside_volatile: 12.01% -> 11.53% (-0.47%) флэт
  - fastutil: 8.54% -> 9.24% (+0.70%) флэт
  - java_util: 7.01% -> 6.74% (-0.27%) флэт
  - paletted: 6.41% -> 6.05% (-0.35%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
