# absorb ROUND (395-a, run 35537446878, branch round-395-a-merge-index, head 7eb5502)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6939260 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6939260 (поллов=5); TPS_exp=2.26; normalized=-7.1%
- GC: young=110, Full=9, total=21.8s, avg=184ms, max=2833ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114847 сэмплов (базлайн 115655)
  - items: 31.17% -> 31.21% (+0.04%) флэт
  - fluid: 16.72% -> 16.19% (-0.53%) флэт
  - broadphase: 15.66% -> 15.97% (+0.31%) флэт
  - nav_ai: 14.16% -> 14.74% (+0.58%) флэт
  - inside_volatile: 12.01% -> 11.79% (-0.22%) флэт
  - fastutil: 8.54% -> 9.72% (+1.19%) РОСТ
  - java_util: 7.01% -> 7.02% (+0.01%) флэт
  - paletted: 6.41% -> 6.40% (-0.01%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
