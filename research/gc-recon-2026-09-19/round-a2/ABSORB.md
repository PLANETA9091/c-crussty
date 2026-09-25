# absorb ROUND (a2, run 36104717346, branch round-455-anchor-2, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6897339 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6897339 (поллов=5); TPS_exp=2.25; normalized=-2.3%
- GC: young=107, Full=9, total=20.1s, avg=173ms, max=2379ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116560 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.05% (-2.12%) спад
  - fluid: 16.72% -> 15.48% (-1.24%) спад
  - broadphase: 15.66% -> 15.34% (-0.32%) флэт
  - nav_ai: 14.16% -> 14.18% (+0.02%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.65%) флэт
  - fastutil: 8.54% -> 8.47% (-0.07%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.19% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
