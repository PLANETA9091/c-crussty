# absorb ROUND (round403stc3, run 35608153237, branch round-403-stagcompc, head 60fe902)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6765263 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6765263 (поллов=6); TPS_exp=2.22; normalized=+5.7%
- GC: young=108, Full=9, total=21.2s, avg=181ms, max=2473ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110528 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.77% (+1.05%) РОСТ
  - broadphase: 15.66% -> 13.91% (-1.75%) спад
  - nav_ai: 14.16% -> 8.81% (-5.36%) спад
  - inside_volatile: 12.01% -> 11.61% (-0.39%) флэт
  - fastutil: 8.54% -> 7.53% (-1.01%) спад
  - java_util: 7.01% -> 7.40% (+0.39%) флэт
  - paletted: 6.41% -> 6.99% (+0.58%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
