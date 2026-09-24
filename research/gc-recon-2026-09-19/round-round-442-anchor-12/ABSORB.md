# absorb ROUND (round-442-anchor-12, run 35957169840, branch round-442-anchor-12, head 335f170)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6623696 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6623696 (поллов=5); TPS_exp=2.19; normalized=+0.3%
- GC: young=112, Full=10, total=23.7s, avg=194ms, max=2457ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117169 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.64% (-1.53%) спад
  - fluid: 16.72% -> 16.09% (-0.62%) флэт
  - broadphase: 15.66% -> 15.25% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.79% (-0.38%) флэт
  - inside_volatile: 12.01% -> 11.66% (-0.35%) флэт
  - fastutil: 8.54% -> 8.40% (-0.14%) флэт
  - java_util: 7.01% -> 6.18% (-0.83%) флэт
  - paletted: 6.41% -> 6.00% (-0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
