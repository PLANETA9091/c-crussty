# absorb ROUND (round-443g-anchor-10, run 36038703794, branch round-443g-anchor-10, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7620461 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7620461 (поллов=5); TPS_exp=2.40; normalized=+4.0%
- GC: young=116, Full=9, total=23.0s, avg=184ms, max=2883ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116905 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.45% (-0.72%) флэт
  - fluid: 16.72% -> 18.21% (+1.49%) РОСТ
  - broadphase: 15.66% -> 15.56% (-0.09%) флэт
  - nav_ai: 14.16% -> 14.02% (-0.14%) флэт
  - inside_volatile: 12.01% -> 12.22% (+0.21%) флэт
  - fastutil: 8.54% -> 8.64% (+0.10%) флэт
  - java_util: 7.01% -> 6.34% (-0.67%) флэт
  - paletted: 6.41% -> 6.39% (-0.01%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
