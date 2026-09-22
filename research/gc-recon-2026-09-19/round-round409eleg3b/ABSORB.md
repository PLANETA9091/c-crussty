# absorb ROUND (round409eleg3b, run 35669112587, branch round-406-e-l3b, head b405fe1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6771554 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6771554 (поллов=6); TPS_exp=2.22; normalized=+12.4%
- GC: young=109, Full=9, total=18.9s, avg=161ms, max=2291ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112856 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.27% (+3.10%) РОСТ
  - fluid: 16.72% -> 17.45% (+0.74%) флэт
  - broadphase: 15.66% -> 15.87% (+0.21%) флэт
  - nav_ai: 14.16% -> 8.61% (-5.55%) спад
  - inside_volatile: 12.01% -> 12.11% (+0.10%) флэт
  - fastutil: 8.54% -> 7.07% (-1.47%) спад
  - java_util: 7.01% -> 7.20% (+0.19%) флэт
  - paletted: 6.41% -> 6.24% (-0.16%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
