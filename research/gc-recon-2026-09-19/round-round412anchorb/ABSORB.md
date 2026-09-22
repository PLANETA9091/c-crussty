# absorb ROUND (round412anchorb, run 35704286802, branch round-412-anchorb, head 3bd07fe)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=4786608 (band OUT) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 4786608 (поллов=5); TPS_exp=1.81; normalized=+5.2%
- GC: young=107, Full=9, total=23.1s, avg=199ms, max=2723ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113285 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.53% (-0.64%) флэт
  - fluid: 16.72% -> 16.82% (+0.10%) флэт
  - broadphase: 15.66% -> 15.96% (+0.30%) флэт
  - nav_ai: 14.16% -> 14.41% (+0.25%) флэт
  - inside_volatile: 12.01% -> 10.70% (-1.30%) спад
  - fastutil: 8.54% -> 9.51% (+0.97%) флэт
  - java_util: 7.01% -> 6.70% (-0.31%) флэт
  - paletted: 6.41% -> 7.12% (+0.71%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
