# absorb ROUND (450b-anchor-20, run 36055382229, branch round-450-anchor-20, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7161318 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 7161318 (поллов=6); TPS_exp=2.31; normalized=+1.9%
- GC: young=107, Full=10, total=22.7s, avg=194ms, max=2426ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116925 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.71% (-1.46%) спад
  - fluid: 16.72% -> 15.63% (-1.09%) спад
  - broadphase: 15.66% -> 15.10% (-0.56%) флэт
  - nav_ai: 14.16% -> 13.69% (-0.47%) флэт
  - inside_volatile: 12.01% -> 11.62% (-0.38%) флэт
  - fastutil: 8.54% -> 8.48% (-0.06%) флэт
  - java_util: 7.01% -> 6.84% (-0.17%) флэт
  - paletted: 6.41% -> 6.16% (-0.25%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
