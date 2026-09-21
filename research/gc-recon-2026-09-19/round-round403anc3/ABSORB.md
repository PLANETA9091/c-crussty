# absorb ROUND (round403anc3, run 35604160858, branch round-403-anchorb, head f19d5f5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6898063 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6898063 (поллов=5); TPS_exp=2.25; normalized=-6.7%
- GC: young=104, Full=9, total=21.1s, avg=187ms, max=2880ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114878 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.82% (-0.35%) флэт
  - fluid: 16.72% -> 16.21% (-0.50%) флэт
  - broadphase: 15.66% -> 16.60% (+0.94%) флэт
  - nav_ai: 14.16% -> 14.25% (+0.08%) флэт
  - inside_volatile: 12.01% -> 11.68% (-0.33%) флэт
  - fastutil: 8.54% -> 8.80% (+0.26%) флэт
  - java_util: 7.01% -> 6.78% (-0.24%) флэт
  - paletted: 6.41% -> 6.33% (-0.07%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
