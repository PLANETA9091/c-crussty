# absorb ROUND (a8-457, run 36131654624, branch round-457-anchor-8, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9004607 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 9004607 (поллов=5); TPS_exp=2.70; normalized=+3.9%
- GC: young=126, Full=10, total=21.7s, avg=160ms, max=2074ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113896 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.95% (-2.22%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 14.46% (-1.19%) спад
  - nav_ai: 14.16% -> 13.77% (-0.40%) флэт
  - inside_volatile: 12.01% -> 10.74% (-1.27%) спад
  - fastutil: 8.54% -> 9.05% (+0.51%) флэт
  - java_util: 7.01% -> 6.46% (-0.55%) флэт
  - paletted: 6.41% -> 6.92% (+0.51%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
