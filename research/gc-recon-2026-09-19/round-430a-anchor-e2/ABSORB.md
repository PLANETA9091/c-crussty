# absorb ROUND (430a-anchor-e2, run 35876742338, branch round-430a-anchor-e2, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7064317 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 7064317 (поллов=5); TPS_exp=2.29; normalized=-3.8%
- GC: young=111, Full=10, total=25.0s, avg=207ms, max=2593ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117199 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.53% (-1.64%) спад
  - fluid: 16.72% -> 15.38% (-1.33%) спад
  - broadphase: 15.66% -> 15.00% (-0.65%) флэт
  - nav_ai: 14.16% -> 13.66% (-0.50%) флэт
  - inside_volatile: 12.01% -> 11.24% (-0.77%) флэт
  - fastutil: 8.54% -> 8.25% (-0.28%) флэт
  - java_util: 7.01% -> 6.88% (-0.13%) флэт
  - paletted: 6.41% -> 6.16% (-0.24%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
