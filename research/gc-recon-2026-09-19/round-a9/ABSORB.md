# absorb ROUND (a9, run 36104780506, branch round-455-anchor-9, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7438628 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7438628 (поллов=5); TPS_exp=2.37; normalized=+9.9%
- GC: young=116, Full=9, total=23.9s, avg=191ms, max=2826ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116878 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.40% (-0.77%) флэт
  - fluid: 16.72% -> 17.88% (+1.16%) РОСТ
  - broadphase: 15.66% -> 15.68% (+0.03%) флэт
  - nav_ai: 14.16% -> 13.77% (-0.40%) флэт
  - inside_volatile: 12.01% -> 11.99% (-0.01%) флэт
  - fastutil: 8.54% -> 8.07% (-0.47%) флэт
  - java_util: 7.01% -> 6.42% (-0.60%) флэт
  - paletted: 6.41% -> 6.54% (+0.13%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
