# absorb ROUND (a21-456w2, run 36116726001, branch round-456-anchor-21, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6922196 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6922196 (поллов=6); TPS_exp=2.26; normalized=-2.5%
- GC: young=107, Full=9, total=21.4s, avg=185ms, max=2844ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116118 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.58% (-0.59%) флэт
  - fluid: 16.72% -> 16.47% (-0.24%) флэт
  - broadphase: 15.66% -> 16.55% (+0.90%) флэт
  - nav_ai: 14.16% -> 14.11% (-0.05%) флэт
  - inside_volatile: 12.01% -> 11.64% (-0.37%) флэт
  - fastutil: 8.54% -> 9.14% (+0.60%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.24% (-0.17%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
