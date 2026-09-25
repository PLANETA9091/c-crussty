# absorb ROUND (a31, run 36108849159, branch round-455-anchor-31, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6901485 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6901485 (поллов=5); TPS_exp=2.25; normalized=-11.2%
- GC: young=107, Full=9, total=21.1s, avg=182ms, max=2463ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116020 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.41% (-1.76%) спад
  - fluid: 16.72% -> 15.73% (-0.98%) флэт
  - broadphase: 15.66% -> 15.34% (-0.32%) флэт
  - nav_ai: 14.16% -> 13.98% (-0.18%) флэт
  - inside_volatile: 12.01% -> 11.14% (-0.87%) флэт
  - fastutil: 8.54% -> 8.54% (-0.00%) флэт
  - java_util: 7.01% -> 6.44% (-0.57%) флэт
  - paletted: 6.41% -> 6.15% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
