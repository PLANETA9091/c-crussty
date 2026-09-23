# absorb ROUND (431-anchor-c, run 35880155662, branch round-431-anchor-c, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7528225 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 7528225 (поллов=5); TPS_exp=2.38; normalized=-20.3%
- GC: young=107, Full=10, total=29.1s, avg=249ms, max=2766ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115124 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.06% (-2.11%) спад
  - fluid: 16.72% -> 16.07% (-0.65%) флэт
  - broadphase: 15.66% -> 14.95% (-0.71%) флэт
  - nav_ai: 14.16% -> 13.30% (-0.86%) флэт
  - inside_volatile: 12.01% -> 10.65% (-1.35%) спад
  - fastutil: 8.54% -> 8.32% (-0.22%) флэт
  - java_util: 7.01% -> 6.96% (-0.06%) флэт
  - paletted: 6.41% -> 7.00% (+0.59%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
