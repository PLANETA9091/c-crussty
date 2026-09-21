# absorb ROUND (round403al2b, run 35611982248, branch round-403-a-leg2, head c8bfbe0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=4893748 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 4893748 (поллов=6); TPS_exp=1.83; normalized=+42.1%
- GC: young=111, Full=9, total=22.9s, avg=191ms, max=3071ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=110362 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.84% (+3.67%) РОСТ
  - fluid: 16.72% -> 19.34% (+2.62%) РОСТ
  - broadphase: 15.66% -> 10.91% (-4.75%) спад
  - nav_ai: 14.16% -> 7.60% (-6.56%) спад
  - inside_volatile: 12.01% -> 12.15% (+0.15%) флэт
  - fastutil: 8.54% -> 7.23% (-1.30%) спад
  - java_util: 7.01% -> 7.37% (+0.36%) флэт
  - paletted: 6.41% -> 7.40% (+1.00%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
