# absorb ROUND (round403jb1, run 35613947014, branch round-403-b-jnibulk, head 3d230ea)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6784909 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6784909 (поллов=5); TPS_exp=2.23; normalized=+7.7%
- GC: young=111, Full=9, total=19.3s, avg=161ms, max=2182ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112561 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.13% (+0.41%) флэт
  - broadphase: 15.66% -> 13.98% (-1.67%) спад
  - nav_ai: 14.16% -> 6.82% (-7.35%) спад
  - inside_volatile: 12.01% -> 12.48% (+0.47%) флэт
  - fastutil: 8.54% -> 7.08% (-1.46%) спад
  - java_util: 7.01% -> 7.52% (+0.51%) флэт
  - paletted: 6.41% -> 6.32% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
