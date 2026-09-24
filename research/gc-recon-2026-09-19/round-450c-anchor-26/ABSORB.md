# absorb ROUND (450c-anchor-26, run 36058456904, branch round-450-anchor-26, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6749311 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6749311 (поллов=6); TPS_exp=2.22; normalized=+1.3%
- GC: young=111, Full=10, total=24.7s, avg=204ms, max=2607ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116764 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.91% (-1.26%) спад
  - fluid: 16.72% -> 15.80% (-0.91%) флэт
  - broadphase: 15.66% -> 15.48% (-0.17%) флэт
  - nav_ai: 14.16% -> 14.29% (+0.13%) флэт
  - inside_volatile: 12.01% -> 11.53% (-0.48%) флэт
  - fastutil: 8.54% -> 8.94% (+0.40%) флэт
  - java_util: 7.01% -> 6.88% (-0.14%) флэт
  - paletted: 6.41% -> 5.85% (-0.55%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
