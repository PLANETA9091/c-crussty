# absorb ROUND (anchor-558, run 36206891054, branch round-463-anchor-558, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8905482 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8905482 (поллов=5); TPS_exp=2.67; normalized=+1.0%
- GC: young=126, Full=10, total=22.0s, avg=162ms, max=2108ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112842 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.69% (-2.48%) спад
  - fluid: 16.72% -> 16.45% (-0.26%) флэт
  - broadphase: 15.66% -> 15.45% (-0.21%) флэт
  - nav_ai: 14.16% -> 13.88% (-0.28%) флэт
  - inside_volatile: 12.01% -> 11.00% (-1.00%) спад
  - fastutil: 8.54% -> 9.45% (+0.92%) флэт
  - java_util: 7.01% -> 6.52% (-0.50%) флэт
  - paletted: 6.41% -> 6.65% (+0.25%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
