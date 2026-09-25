# absorb ROUND (a4, run 36104735624, branch round-455-anchor-4, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=9112967 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 9112967 (поллов=5); TPS_exp=2.72; normalized=-4.3%
- GC: young=119, Full=9, total=23.8s, avg=186ms, max=2681ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116969 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.93% (-2.24%) спад
  - fluid: 16.72% -> 16.64% (-0.08%) флэт
  - broadphase: 15.66% -> 13.80% (-1.85%) спад
  - nav_ai: 14.16% -> 13.93% (-0.23%) флэт
  - inside_volatile: 12.01% -> 11.99% (-0.02%) флэт
  - fastutil: 8.54% -> 8.81% (+0.27%) флэт
  - java_util: 7.01% -> 6.91% (-0.10%) флэт
  - paletted: 6.41% -> 7.40% (+0.99%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
