# absorb ROUND (anchor420c, run 35789966747, branch round-420-anchorc, head 41456af)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6295959 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6295959 (поллов=5); TPS_exp=2.12; normalized=-1.2%
- GC: young=116, Full=9, total=23.9s, avg=191ms, max=2614ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116311 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.14% (-2.03%) спад
  - fluid: 16.72% -> 16.81% (+0.10%) флэт
  - broadphase: 15.66% -> 14.70% (-0.95%) флэт
  - nav_ai: 14.16% -> 13.79% (-0.37%) флэт
  - inside_volatile: 12.01% -> 11.06% (-0.95%) флэт
  - fastutil: 8.54% -> 8.80% (+0.27%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.79% (+0.39%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
