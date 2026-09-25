# absorb ROUND (a9-456, run 36112161416, branch round-456-anchor-9, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6590823 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6590823 (поллов=5); TPS_exp=2.19; normalized=+5.2%
- GC: young=114, Full=9, total=23.5s, avg=191ms, max=2617ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114950 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.27% (-0.90%) флэт
  - fluid: 16.72% -> 17.24% (+0.53%) флэт
  - broadphase: 15.66% -> 15.12% (-0.53%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.36%) флэт
  - inside_volatile: 12.01% -> 11.00% (-1.01%) спад
  - fastutil: 8.54% -> 8.48% (-0.06%) флэт
  - java_util: 7.01% -> 6.18% (-0.83%) флэт
  - paletted: 6.41% -> 7.52% (+1.12%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
