# absorb ROUND (round-435-cce-l2r7, run 35924529413, branch round-435-cce-l2r7, head 513c483)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6720015 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6720015 (поллов=5); TPS_exp=2.21; normalized=+8.4%
- GC: young=109, Full=9, total=22.8s, avg=193ms, max=2786ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104117 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.19% (+0.48%) флэт
  - broadphase: 15.66% -> 9.21% (-6.44%) спад
  - nav_ai: 14.16% -> 3.13% (-11.03%) спад
  - inside_volatile: 12.01% -> 15.95% (+3.94%) РОСТ
  - fastutil: 8.54% -> 6.70% (-1.84%) спад
  - java_util: 7.01% -> 8.97% (+1.96%) РОСТ
  - paletted: 6.41% -> 6.07% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
