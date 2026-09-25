# absorb ROUND (anchor458-34, run 36146449192, branch round-458-anchor-34, head 96cc270)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=5429542 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 5429542 (поллов=6); TPS_exp=1.94; normalized=+15.9%
- GC: young=109, Full=9, total=22.0s, avg=186ms, max=3010ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115984 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.51% (-0.67%) флэт
  - fluid: 16.72% -> 16.21% (-0.50%) флэт
  - broadphase: 15.66% -> 15.64% (-0.01%) флэт
  - nav_ai: 14.16% -> 14.42% (+0.26%) флэт
  - inside_volatile: 12.01% -> 11.67% (-0.34%) флэт
  - fastutil: 8.54% -> 9.24% (+0.70%) флэт
  - java_util: 7.01% -> 7.13% (+0.12%) флэт
  - paletted: 6.41% -> 6.25% (-0.16%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
