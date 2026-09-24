# absorb ROUND (round-435b-anchor-b35, run 35927387221, branch round-435b-anchor-b35, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6745448 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6745448 (поллов=5); TPS_exp=2.22; normalized=+3.6%
- GC: young=118, Full=9, total=23.6s, avg=186ms, max=2529ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115840 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.72% (-1.45%) спад
  - fluid: 16.72% -> 16.90% (+0.19%) флэт
  - broadphase: 15.66% -> 14.39% (-1.26%) спад
  - nav_ai: 14.16% -> 13.35% (-0.81%) флэт
  - inside_volatile: 12.01% -> 11.24% (-0.76%) флэт
  - fastutil: 8.54% -> 8.67% (+0.14%) флэт
  - java_util: 7.01% -> 6.73% (-0.28%) флэт
  - paletted: 6.41% -> 7.00% (+0.60%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
