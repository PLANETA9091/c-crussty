# absorb ROUND (a21, run 36108659850, branch round-455-anchor-21, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7186583 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7186583 (поллов=5); TPS_exp=2.31; normalized=-4.9%
- GC: young=112, Full=10, total=23.4s, avg=192ms, max=2360ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116416 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.54% (-0.64%) флэт
  - fluid: 16.72% -> 16.07% (-0.65%) флэт
  - broadphase: 15.66% -> 15.38% (-0.27%) флэт
  - nav_ai: 14.16% -> 14.05% (-0.11%) флэт
  - inside_volatile: 12.01% -> 11.66% (-0.35%) флэт
  - fastutil: 8.54% -> 9.54% (+1.00%) флэт
  - java_util: 7.01% -> 6.81% (-0.21%) флэт
  - paletted: 6.41% -> 6.66% (+0.25%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
