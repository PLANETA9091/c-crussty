# absorb ROUND (a3, run 36104726524, branch round-455-anchor-3, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7061411 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7061411 (поллов=6); TPS_exp=2.29; normalized=+0.6%
- GC: young=108, Full=9, total=20.5s, avg=175ms, max=2379ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116610 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.79% (-1.38%) спад
  - fluid: 16.72% -> 15.78% (-0.93%) флэт
  - broadphase: 15.66% -> 15.36% (-0.29%) флэт
  - nav_ai: 14.16% -> 13.74% (-0.42%) флэт
  - inside_volatile: 12.01% -> 11.63% (-0.37%) флэт
  - fastutil: 8.54% -> 8.76% (+0.22%) флэт
  - java_util: 7.01% -> 6.42% (-0.59%) флэт
  - paletted: 6.41% -> 6.30% (-0.11%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
