# absorb ROUND (mega421b, run 35806668516, branch round-421-mgb, head 99ffefa)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7175157 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 7175157 (поллов=6); TPS_exp=2.31; normalized=+1.7%
- GC: young=1128, Full=9, total=27.2s, avg=24ms, max=2447ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107719 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.19% (-0.53%) флэт
  - broadphase: 15.66% -> 15.28% (-0.38%) флэт
  - nav_ai: 14.16% -> 9.18% (-4.99%) спад
  - inside_volatile: 12.01% -> 12.40% (+0.39%) флэт
  - fastutil: 8.54% -> 8.55% (+0.01%) флэт
  - java_util: 7.01% -> 8.40% (+1.38%) РОСТ
  - paletted: 6.41% -> 5.51% (-0.89%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
