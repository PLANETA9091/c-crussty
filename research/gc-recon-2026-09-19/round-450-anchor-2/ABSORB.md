# absorb ROUND (450-anchor-2, run 36050509768, branch round-450-anchor-2, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6431555 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6431555 (поллов=5); TPS_exp=2.15; normalized=-2.5%
- GC: young=114, Full=10, total=26.8s, avg=216ms, max=2644ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115743 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.36% (-1.81%) спад
  - fluid: 16.72% -> 17.35% (+0.64%) флэт
  - broadphase: 15.66% -> 15.05% (-0.61%) флэт
  - nav_ai: 14.16% -> 13.57% (-0.59%) флэт
  - inside_volatile: 12.01% -> 11.10% (-0.91%) флэт
  - fastutil: 8.54% -> 8.56% (+0.02%) флэт
  - java_util: 7.01% -> 6.27% (-0.75%) флэт
  - paletted: 6.41% -> 7.01% (+0.60%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
