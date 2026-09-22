# absorb ROUND (round413anchora, run 35714990661, branch round-413-anchora, head dab32b4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6862504 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6862504 (поллов=5); TPS_exp=2.24; normalized=-6.4%
- GC: young=108, Full=8, total=21.1s, avg=182ms, max=2463ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115256 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 16.10% (-0.61%) флэт
  - broadphase: 15.66% -> 16.37% (+0.71%) флэт
  - nav_ai: 14.16% -> 14.41% (+0.24%) флэт
  - inside_volatile: 12.01% -> 11.05% (-0.96%) флэт
  - fastutil: 8.54% -> 8.62% (+0.09%) флэт
  - java_util: 7.01% -> 6.67% (-0.34%) флэт
  - paletted: 6.41% -> 6.36% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
