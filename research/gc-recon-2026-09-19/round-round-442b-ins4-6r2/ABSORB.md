# absorb ROUND (round-442b-ins4-6r2, run 35960102441, branch round-442b-ins4-6r2, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8899646 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.30 @ 8899646 (поллов=5); TPS_exp=2.67; normalized=+23.4%
- GC: young=132, Full=10, total=19.3s, avg=136ms, max=2189ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104704 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.30% (-0.42%) флэт
  - broadphase: 15.66% -> 9.16% (-6.49%) спад
  - nav_ai: 14.16% -> 3.79% (-10.37%) спад
  - inside_volatile: 12.01% -> 15.91% (+3.90%) РОСТ
  - fastutil: 8.54% -> 6.38% (-2.16%) спад
  - java_util: 7.01% -> 8.19% (+1.18%) РОСТ
  - paletted: 6.41% -> 6.00% (-0.40%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
