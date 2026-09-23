# absorb ROUND (anchor422b, run 35809267976, branch round-422-anchorb, head 57a2e67)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6724897 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6724897 (поллов=5); TPS_exp=2.22; normalized=-9.7%
- GC: young=106, Full=9, total=19.8s, avg=172ms, max=2562ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116234 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.45% (-1.72%) спад
  - fluid: 16.72% -> 15.71% (-1.00%) спад
  - broadphase: 15.66% -> 15.42% (-0.23%) флэт
  - nav_ai: 14.16% -> 14.06% (-0.10%) флэт
  - inside_volatile: 12.01% -> 10.99% (-1.01%) спад
  - fastutil: 8.54% -> 8.61% (+0.07%) флэт
  - java_util: 7.01% -> 6.89% (-0.12%) флэт
  - paletted: 6.41% -> 6.06% (-0.34%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
