# absorb ROUND (431-anchor-a, run 35880090997, branch round-431-anchor-a, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7115923 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 7115923 (поллов=6); TPS_exp=2.30; normalized=-6.4%
- GC: young=111, Full=10, total=25.3s, avg=209ms, max=2766ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116469 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.75% (-1.43%) спад
  - fluid: 16.72% -> 16.06% (-0.66%) флэт
  - broadphase: 15.66% -> 15.24% (-0.42%) флэт
  - nav_ai: 14.16% -> 14.15% (-0.02%) флэт
  - inside_volatile: 12.01% -> 11.22% (-0.78%) флэт
  - fastutil: 8.54% -> 8.82% (+0.29%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.23% (-0.18%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
