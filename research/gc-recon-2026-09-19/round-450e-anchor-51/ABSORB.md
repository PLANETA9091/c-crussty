# absorb ROUND (450e-anchor-51, run 36066588294, branch round-450-anchor-51, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6632835 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6632835 (поллов=5); TPS_exp=2.20; normalized=-4.4%
- GC: young=106, Full=9, total=23.0s, avg=200ms, max=2942ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115470 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.77% (-1.40%) спад
  - fluid: 16.72% -> 16.85% (+0.14%) флэт
  - broadphase: 15.66% -> 15.00% (-0.66%) флэт
  - nav_ai: 14.16% -> 13.35% (-0.81%) флэт
  - inside_volatile: 12.01% -> 10.87% (-1.14%) спад
  - fastutil: 8.54% -> 8.66% (+0.12%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 6.79% (+0.38%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
