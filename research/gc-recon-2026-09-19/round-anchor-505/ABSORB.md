# absorb ROUND (anchor-505, run 36206826407, branch round-463-anchor-505, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8974605 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8974605 (поллов=5); TPS_exp=2.69; normalized=+0.4%
- GC: young=126, Full=10, total=21.5s, avg=158ms, max=2064ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113810 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.93% (-2.24%) спад
  - fluid: 16.72% -> 15.98% (-0.74%) флэт
  - broadphase: 15.66% -> 15.15% (-0.51%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.67%) флэт
  - inside_volatile: 12.01% -> 10.72% (-1.28%) спад
  - fastutil: 8.54% -> 8.77% (+0.24%) флэт
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 6.95% (+0.54%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
