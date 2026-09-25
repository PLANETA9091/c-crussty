# absorb ROUND (a20, run 36108648904, branch round-455-anchor-20, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7079697 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 7079697 (поллов=6); TPS_exp=2.29; normalized=-1.7%
- GC: young=101, Full=9, total=19.8s, avg=180ms, max=2465ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116274 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.08% (-2.09%) спад
  - fluid: 16.72% -> 15.58% (-1.13%) спад
  - broadphase: 15.66% -> 16.00% (+0.34%) флэт
  - nav_ai: 14.16% -> 13.97% (-0.20%) флэт
  - inside_volatile: 12.01% -> 11.08% (-0.93%) флэт
  - fastutil: 8.54% -> 8.60% (+0.07%) флэт
  - java_util: 7.01% -> 6.75% (-0.26%) флэт
  - paletted: 6.41% -> 6.06% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
