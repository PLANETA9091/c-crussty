# absorb ROUND (round-437-chk-1, run 35935276961, branch round-437-chk-1, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6975347 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6975347 (поллов=5); TPS_exp=2.27; normalized=+1.4%
- GC: young=101, Full=9, total=19.3s, avg=176ms, max=2713ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104094 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.08% (-0.63%) флэт
  - broadphase: 15.66% -> 10.08% (-5.58%) спад
  - nav_ai: 14.16% -> 3.02% (-11.14%) спад
  - inside_volatile: 12.01% -> 15.70% (+3.69%) РОСТ
  - fastutil: 8.54% -> 6.88% (-1.66%) спад
  - java_util: 7.01% -> 8.76% (+1.75%) РОСТ
  - paletted: 6.41% -> 5.03% (-1.37%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
