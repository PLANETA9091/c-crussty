# absorb ROUND (anchor424c2, run 35829367357, branch round-424-anchorc2, head a8ef2fc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6765225 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6765225 (поллов=5); TPS_exp=2.22; normalized=-1.1%
- GC: young=111, Full=8, total=21.0s, avg=177ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116367 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.48% (-1.69%) спад
  - fluid: 16.72% -> 15.56% (-1.15%) спад
  - broadphase: 15.66% -> 15.03% (-0.63%) флэт
  - nav_ai: 14.16% -> 13.30% (-0.87%) флэт
  - inside_volatile: 12.01% -> 11.72% (-0.29%) флэт
  - fastutil: 8.54% -> 8.60% (+0.07%) флэт
  - java_util: 7.01% -> 6.24% (-0.77%) флэт
  - paletted: 6.41% -> 6.06% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
