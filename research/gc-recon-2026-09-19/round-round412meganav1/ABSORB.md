# absorb ROUND (round412meganav1, run 35705650726, branch round-412-meganav1, head f579c1c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6725119 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6725119 (поллов=5); TPS_exp=2.22; normalized=+17.4%
- GC: young=106, Full=9, total=18.1s, avg=157ms, max=2201ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107629 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.02% (-0.70%) флэт
  - broadphase: 15.66% -> 13.91% (-1.74%) спад
  - nav_ai: 14.16% -> 9.04% (-5.12%) спад
  - inside_volatile: 12.01% -> 12.18% (+0.17%) флэт
  - fastutil: 8.54% -> 7.52% (-1.01%) спад
  - java_util: 7.01% -> 7.54% (+0.53%) флэт
  - paletted: 6.41% -> 5.73% (-0.68%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
