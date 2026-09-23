# absorb ROUND (brain421l3, run 35804783057, branch round-421-a-brain-l3, head 3e8c431)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6752770 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6752770 (поллов=5); TPS_exp=2.22; normalized=+17.1%
- GC: young=1134, Full=9, total=24.9s, avg=22ms, max=2204ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107140 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.26% (-0.46%) флэт
  - broadphase: 15.66% -> 14.16% (-1.50%) спад
  - nav_ai: 14.16% -> 8.87% (-5.29%) спад
  - inside_volatile: 12.01% -> 12.18% (+0.18%) флэт
  - fastutil: 8.54% -> 8.53% (-0.01%) флэт
  - java_util: 7.01% -> 8.55% (+1.54%) РОСТ
  - paletted: 6.41% -> 5.79% (-0.61%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
