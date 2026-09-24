# absorb ROUND (450-anchor-3, run 36050524528, branch round-450-anchor-3, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6428124 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6428124 (поллов=5); TPS_exp=2.15; normalized=+2.2%
- GC: young=109, Full=9, total=23.4s, avg=199ms, max=2714ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115213 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.14% (-2.03%) спад
  - fluid: 16.72% -> 16.26% (-0.46%) флэт
  - broadphase: 15.66% -> 15.43% (-0.23%) флэт
  - nav_ai: 14.16% -> 13.62% (-0.54%) флэт
  - inside_volatile: 12.01% -> 10.77% (-1.24%) спад
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.85% (+0.44%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
