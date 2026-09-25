# absorb ROUND (a15-457, run 36131731227, branch round-457-anchor-15, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6813110 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6813110 (поллов=6); TPS_exp=2.23; normalized=+9.7%
- GC: young=113, Full=9, total=20.8s, avg=170ms, max=2409ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116886 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.53% (-1.64%) спад
  - fluid: 16.72% -> 15.58% (-1.14%) спад
  - broadphase: 15.66% -> 15.57% (-0.09%) флэт
  - nav_ai: 14.16% -> 14.32% (+0.16%) флэт
  - inside_volatile: 12.01% -> 11.19% (-0.81%) флэт
  - fastutil: 8.54% -> 9.36% (+0.83%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 5.97% (-0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
