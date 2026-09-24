# absorb ROUND (450c-anchor-22, run 36058408460, branch round-450-anchor-22, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6749908 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6749908 (поллов=5); TPS_exp=2.22; normalized=+3.6%
- GC: young=117, Full=10, total=27.5s, avg=217ms, max=2891ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116273 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.58% (-1.59%) спад
  - fluid: 16.72% -> 16.86% (+0.14%) флэт
  - broadphase: 15.66% -> 14.85% (-0.81%) флэт
  - nav_ai: 14.16% -> 13.45% (-0.71%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.65%) флэт
  - fastutil: 8.54% -> 8.27% (-0.26%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.84% (+0.44%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
