# absorb ROUND (431b-anchor-b, run 35883932879, branch round-431b-anchor-b, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8880703 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8880703 (поллов=5); TPS_exp=2.67; normalized=-2.6%
- GC: young=122, Full=10, total=21.3s, avg=161ms, max=2101ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113833 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.69% (-2.49%) спад
  - fluid: 16.72% -> 16.55% (-0.17%) флэт
  - broadphase: 15.66% -> 14.84% (-0.81%) флэт
  - nav_ai: 14.16% -> 13.90% (-0.26%) флэт
  - inside_volatile: 12.01% -> 10.67% (-1.34%) спад
  - fastutil: 8.54% -> 8.96% (+0.42%) флэт
  - java_util: 7.01% -> 6.73% (-0.28%) флэт
  - paletted: 6.41% -> 7.02% (+0.62%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
