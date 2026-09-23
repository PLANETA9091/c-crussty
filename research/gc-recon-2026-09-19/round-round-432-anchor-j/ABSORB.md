# absorb ROUND (round-432-anchor-j, run 35896531300, branch round-432-anchor-j, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6602780 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6602780 (поллов=5); TPS_exp=2.19; normalized=+0.5%
- GC: young=106, Full=10, total=23.5s, avg=202ms, max=2387ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116606 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.91% (-1.26%) спад
  - fluid: 16.72% -> 16.10% (-0.62%) флэт
  - broadphase: 15.66% -> 15.46% (-0.20%) флэт
  - nav_ai: 14.16% -> 13.73% (-0.43%) флэт
  - inside_volatile: 12.01% -> 11.11% (-0.89%) флэт
  - fastutil: 8.54% -> 8.29% (-0.24%) флэт
  - java_util: 7.01% -> 6.69% (-0.32%) флэт
  - paletted: 6.41% -> 6.29% (-0.11%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
