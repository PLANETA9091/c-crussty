# absorb ROUND (451b-anchor-15, run 36073603913, branch round-451-anchor-15, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7030500 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7030500 (поллов=6); TPS_exp=2.28; normalized=+0.9%
- GC: young=111, Full=9, total=21.3s, avg=177ms, max=2409ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116508 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.97% (-1.20%) спад
  - fluid: 16.72% -> 15.99% (-0.72%) флэт
  - broadphase: 15.66% -> 14.82% (-0.84%) флэт
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 11.53% (-0.47%) флэт
  - fastutil: 8.54% -> 8.71% (+0.17%) флэт
  - java_util: 7.01% -> 6.67% (-0.34%) флэт
  - paletted: 6.41% -> 6.47% (+0.06%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
