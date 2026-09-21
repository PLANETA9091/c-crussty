# absorb ROUND (round403stc2, run 35604321849, branch round-403-stcomp2, head 60fe902)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7120479 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 7120479 (поллов=5); TPS_exp=2.30; normalized=+26.2%
- GC: young=118, Full=9, total=22.0s, avg=174ms, max=2587ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113181 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 19.07% (+2.35%) РОСТ
  - broadphase: 15.66% -> 14.82% (-0.84%) флэт
  - nav_ai: 14.16% -> 10.36% (-3.81%) спад
  - inside_volatile: 12.01% -> 12.55% (+0.54%) флэт
  - fastutil: 8.54% -> 7.52% (-1.02%) спад
  - java_util: 7.01% -> 6.51% (-0.50%) флэт
  - paletted: 6.41% -> 6.64% (+0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
