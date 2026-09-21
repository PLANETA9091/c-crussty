# absorb ROUND (round405cleg1, run 35659765756, branch round-405-c-l1, head d7caf96)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6944967 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6944967 (поллов=6); TPS_exp=2.26; normalized=+3.9%
- GC: young=114, Full=9, total=21.3s, avg=173ms, max=2421ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115710 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.99% (-1.18%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 15.23% (-0.43%) флэт
  - nav_ai: 14.16% -> 14.10% (-0.06%) флэт
  - inside_volatile: 12.01% -> 11.89% (-0.11%) флэт
  - fastutil: 8.54% -> 8.58% (+0.04%) флэт
  - java_util: 7.01% -> 6.56% (-0.45%) флэт
  - paletted: 6.41% -> 6.15% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
