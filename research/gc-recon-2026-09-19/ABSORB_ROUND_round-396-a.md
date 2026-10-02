# absorb ROUND (396-a, run 35536885888, branch round-396-a-items_index, head 3c068f2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6825137 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6825137 (поллов=5); TPS_exp=2.24; normalized=+2.9%
- GC: young=110, Full=9, total=21.8s, avg=183ms, max=2714ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114964 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.95% (-0.22%) флэт
  - fluid: 16.72% -> 16.10% (-0.62%) флэт
  - broadphase: 15.66% -> 15.73% (+0.07%) флэт
  - nav_ai: 14.16% -> 13.98% (-0.18%) флэт
  - inside_volatile: 12.01% -> 11.68% (-0.33%) флэт
  - fastutil: 8.54% -> 9.23% (+0.69%) флэт
  - java_util: 7.01% -> 7.30% (+0.29%) флэт
  - paletted: 6.41% -> 6.38% (-0.02%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
