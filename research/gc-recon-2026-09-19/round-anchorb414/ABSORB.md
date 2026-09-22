# absorb ROUND (anchorb414, run 35725992222, branch round-414-anchorb, head 5b4bf42)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6701329 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 6701329 (поллов=6); TPS_exp=2.21; normalized=+10.9%
- GC: young=111, Full=9, total=21.1s, avg=176ms, max=2729ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115270 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.48% (-0.69%) флэт
  - fluid: 16.72% -> 15.87% (-0.84%) флэт
  - broadphase: 15.66% -> 16.27% (+0.62%) флэт
  - nav_ai: 14.16% -> 14.09% (-0.08%) флэт
  - inside_volatile: 12.01% -> 11.57% (-0.44%) флэт
  - fastutil: 8.54% -> 8.93% (+0.39%) флэт
  - java_util: 7.01% -> 6.81% (-0.20%) флэт
  - paletted: 6.41% -> 6.11% (-0.29%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
