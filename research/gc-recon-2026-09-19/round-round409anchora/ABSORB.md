# absorb ROUND (round409anchora, run 35666723979, branch round-408-anchora, head 010a07e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6833921 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6833921 (поллов=6); TPS_exp=2.24; normalized=+0.5%
- GC: young=115, Full=9, total=21.2s, avg=171ms, max=2411ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113919 сэмплов (базлайн 115655)
  - items: 31.17% -> 31.11% (-0.06%) флэт
  - fluid: 16.72% -> 16.70% (-0.02%) флэт
  - broadphase: 15.66% -> 15.36% (-0.30%) флэт
  - nav_ai: 14.16% -> 14.17% (+0.01%) флэт
  - inside_volatile: 12.01% -> 11.98% (-0.03%) флэт
  - fastutil: 8.54% -> 8.89% (+0.35%) флэт
  - java_util: 7.01% -> 6.94% (-0.07%) флэт
  - paletted: 6.41% -> 6.21% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
