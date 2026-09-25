# absorb ROUND (a25, run 36108702089, branch round-455-anchor-25, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7551035 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7551035 (поллов=5); TPS_exp=2.39; normalized=+8.8%
- GC: young=120, Full=9, total=24.1s, avg=187ms, max=2766ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117038 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.23% (-0.94%) флэт
  - fluid: 16.72% -> 18.08% (+1.37%) РОСТ
  - broadphase: 15.66% -> 15.58% (-0.08%) флэт
  - nav_ai: 14.16% -> 13.95% (-0.21%) флэт
  - inside_volatile: 12.01% -> 12.12% (+0.11%) флэт
  - fastutil: 8.54% -> 8.38% (-0.16%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.45% (+0.04%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
